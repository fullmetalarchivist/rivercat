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

### More
The construction of the _rivercat_ cipher is explained in more detail in the blog post [here](https://fullmetalarchivist.github.io/archive/posts/rivercat).

