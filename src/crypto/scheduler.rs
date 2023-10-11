use crate::constants::{RCON, S_BOX};
use crate::block::Block128;

pub struct KeyExpandedBlock {
    key: [u32; 44],
}

pub struct KeyExpander;

impl KeyExpander {
    pub fn expand_keys(input: Block128) -> KeyExpandedBlock {
        KeyExpandedBlock { key: aes_key_schedule(&input.data) }
    }
}

pub trait Scheduler<B> 
where
    B: crate::block::Block,
{
    fn next_key(&mut self) -> Block128;
    fn set_position(&mut self, i: usize);
    fn get_key_at_position(&mut self, i: usize) -> Block128;
}

pub struct IterativeRijndaelScheduler {
    k_i: KeyExpandedBlock,
    i: usize,
}

impl IterativeRijndaelScheduler {
    pub fn new(init_key: Block128) -> Self {
        log::info!("computing expanded Rijndael key expansions");
        let expanded = aes_key_schedule(&init_key.data);
        log::debug!("expanded keys: {:x?}", expanded);
        let k_i = KeyExpandedBlock { key: expanded };
        IterativeRijndaelScheduler { k_i, i: 0 }
    }
}

impl Scheduler<Block128> for IterativeRijndaelScheduler {
    fn set_position(&mut self, i: usize) {
        self.i = i;
    }
    fn get_key_at_position(&mut self, i: usize) -> Block128 {
        log::debug!("(rivercat) getting precomputed expansion block at index {} for K_{}", i, i);
        let mut block128 = [0u8; 16];
        for (j, &num) in self.k_i.key[i..i+4].iter().enumerate() {
            let start_index = j * 4;
            block128[start_index] = ((num >> 24) & 0xFF) as u8;
            block128[start_index + 1] = ((num >> 16) & 0xFF) as u8;
            block128[start_index + 2] = ((num >> 8) & 0xFF) as u8;
            block128[start_index + 3] = (num & 0xFF) as u8;
        }
        let key = Block128::from(block128);
        key
    }
    fn next_key(&mut self) -> Block128 {
        log::debug!("using the next 128-bit precomputed expansion block");
        // use the next 128-bit block precomputed from the Rijndael key expansion
        if (self.i + 3) >= self.k_i.key.len() {
            self.i = 0;
        }
        let next_block = [
            self.k_i.key[self.i],
            self.k_i.key[self.i + 1],
            self.k_i.key[self.i + 2],
            self.k_i.key[self.i + 3],
        ];
        self.i += 4;
        let mut block128 = [0u8; 16];
        for (i, &num) in next_block.iter().enumerate() {
            let start_index = i * 4;
            block128[start_index] = ((num >> 24) & 0xFF) as u8;
            block128[start_index + 1] = ((num >> 16) & 0xFF) as u8;
            block128[start_index + 2] = ((num >> 8) & 0xFF) as u8;
            block128[start_index + 3] = (num & 0xFF) as u8;
        }
        let k_i = Block128::from(block128);
        k_i
    }
}

pub(crate) fn sub_word(word: u32) -> u32 {
    let mut result = 0;
    for i in 0..4 {
        let byte = (word >> (8 * i)) as u8;
        let subbed = s_box_substitution(byte);
        result |= (subbed as u32) << (8 * i);
    }
    return result;
}

pub fn rot_word(word: u32) -> u32 {
    return (word << 8) | (word >> 24);
}

pub fn s_box_substitution(byte: u8) -> u8 {
    let row = (byte >> 4) as usize;
    let col = (byte & 0x0F) as usize;
    return S_BOX[row][col];
}

pub fn aes_key_schedule(key: &[u8; 16]) -> [u32; 44] {
    let mut w: [u32; 44] = [0; 44];
    log::debug!("(rijndael) expanding 128-bit root key into 44 32-bit words");

    for i in 0..4 {
        w[i] = u32::from_be_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
    }

    for i in 4..44 {
        let mut temp = w[i - 1];
        if i % 4 == 0 {
            temp = sub_word(rot_word(temp)) ^ (RCON[i / 4 - 1] as u32) << 24;
        }
        w[i] = w[i - 4] ^ temp;
    }

    return w;
}

#[cfg(test)]
mod scheduler_test {
    use super::*;

    #[test]
    fn test_s_box_sub() {
        pretty_env_logger::try_init().ok();

        assert_eq!(s_box_substitution(0xdc), 0x86);
    }

}
