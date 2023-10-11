# Rivercat Cipher

Rivercat is a peculiar block cipher constructed using [Feistel Networks](https://en.wikipedia.org/wiki/Feistel_cipher) with a slightly modified [Rijndael (AES) key schedule](https://cryptography.fandom.com/wiki/Rijndael_key_schedule). This was soley created to deepen my understanding of the mechanisms that make block ciphers secure, and is therefore, not audited nor gaurunteed to be cryptographically secure.

## Construction
Rivercat is not [Rijndael](https://csrc.nist.gov/csrc/media/projects/cryptographic-standards-and-guidelines/documents/aes-development/rijndael-ammended.pdf)! Instead of using [SPNs](https://en.wikipedia.org/wiki/Substitution%E2%80%93permutation_network) to achieve diffusion, _rivercat_ uses Feistel ciphers.
## Motivation
As I am learning more basic cryptography I became really interested in the Feistel construction and its simplicity. It felt nice to implement it in code but I wasn't implementing a secure cipher. This is because the 
security of a Feistel cipher is defined by the PRF (round function) and the key scheduler, and I wasn't implementing these mechanisms yet. Soon enough in my other studies I came across the Rijndael cipher and its PRN approach
that felt much more difficult to readily implement in code. However, I thought what if I could use this easier-to-understand part of Rijndael (the key scheduler) and slot it into my Feistal ciphers? This would solve one of my
security concerns with them as well as help me learn more about Rijndael simultaneously. The general motiviation behind _rivercat_ is challenging myself to gain a deeper understanding of the primitives of block ciphers, as well 
as challenge myself to create one, as hacky as it may be; so I can also learn to perform cryptanalysis on it to determine design flaws and fix them.

### The Feistel Construction
_Feistel_ ciphers (or _Fiestel networks_) are a permutation component utilized by many block ciphers, from DES to newer ones such as [Camillia](https://en.wikipedia.org/wiki/Camellia_(cipher)) and [Blowfish](https://en.wikipedia.org/wiki/Blowfish_(cipher)).
Feistel ciphers are good on bare metal because their entire construction is guaranteed to be invertible, nearly halving the required circuit size. 

> _An important advantage of Feistel networks compared to other cipher designs such as substitution–permutation networks* is that the entire operation 
is guaranteed to be invertible (that is, encrypted data can be decrypted), even if the round function is not itself invertible._
> * *Such as Rijndael
> https://en.wikipedia.org/wiki/Feistel_cipher#Design

The construction is as follows:

#### Encryption
Let $F$ be the round function, $\oplus$ denote the bitwise XOR operator, and let $K_0,K_1,...,K_n$ be the subkeys for the rounds $0,1,...,n$. Then the _forward_ operation is as follows,
1. Split the plaintext block into two equal pieces: $(L_0, R_0)$
2. For each round $i=0,1,...,n$ compute 
$$L_{i+1}=R_i$$
$$R_{i+1}=L_{i}\oplus F(R_i, K_i)$$
3. Output ciphertext: $(R_{n+1},L_{n+1})$

#### Decryption
Since the construction is entirely invertible, decryption is straightforward:
1. For $i=n,n-1,...,0$, compute
   $$R_i=L_{i+1},$$
   $$L_i=R_{i+1}\oplus F(L_{i+1},K_i)$$
   > Note that $i$ is reversed. This is essentially stepping through the Feistel network backwards. Also note that $i$ is used to index the round key $K_i$, and thus it follows
   > that the round keys for decryption are defined as $K_n,K_{n-1},...K_0$.
3. Output plaintext: $(L_0,R_0)$

### Rijndael Key Schedule
Rivercat (as of now) uses 128-bit keys, let $N$ be the length of a round key $K_i$ in 32-bit words (e.g., $N=4$ for a 128-bit key). Let $R$ be the number of round keys needed; this is $11$ in AES-128 so _rivercat_ chose it
for a starting point, and let $W_0,W_1,...,W_{4R-1}$ be the 32-bit words of the expanded key. Then the Rijndael key schedule is for $i=0...4R-1$: Let $K_E$ be a key expansion of $K$,
* Initialize a starting key $K_0$
* Compute $K_E$,
  * Fill the first four 32-bit words (128-bits) with the bits of $K_0$ in big endian order
  * Expansion: the remaining words are filled in a loop that iterates from 4 to $4R-1$
    * Use the `sub_word` and `rot_word` to perform S-Box substitutions and bit shifting to generate expanded keys. See [here](https://github.com/phasewalk1/rivercat/blob/master/src/crypto/scheduler.rs#L83). 
...

### Rivercat
_Rivercat_'s work involves applying the Rijndael key schedule to a Feistel cipher over 11 rounds, in an attempt to create a hacky implementation of an AES-128 that does not use PRNs. The _rivercat_ cipher 
precomputes the Rijndael expansions of a 128-bit key and then iteratively uses them through a fixed number of rounds through a Feistel construction. It can be said that _rivercat128_ then has $R=11$ rounds; also note that we use the notation $K_E$ to denote the Rijndael key expansion (44 32-bit words). Rivercat precomputes $K_E\leftarrow\text{expand}(K_0)$ where $K_0$ is some 128-bit initial key and $\text{expand}$ is Rijndael key expansion. What it does to use these precomputated
keys in Feistel ciphers is quite a hack really, and one thing I know I need to fix. For each round $i=0,...,11$ in the Feistel cipher, we need a $K_i$ from the scheduler. $K_E$ contains 44 32-bit words ($32\cdot 44=1408=11\cdot 128$), so we have 11 rounds worth of 128-bit $K_i$ to work with. Where $B$ is an input block of 256-bits, the _rivercat_ cipher performs encryption as
1. Split $B$ into equal halves $L_i, R_i$
2. For $i=0,...,11$:
   $$L_{i+1}=R_i$$
   $$R_{i+1}=L_i\oplus F(R_i, K_{E_i})$$
3. Output ciphertext: $(R_{n+1}, L_{n+1})$

Decryption is simple following the Feistel cipher backwards, for each round $i=11,...,0$
$$R_i=L_{i+1}$$
$$L_i=R_{i+1}\oplus F(L_{i+1}, K_{E_i})$$
Output plaintext: $(R_{n+1}, L_{n+1})$

