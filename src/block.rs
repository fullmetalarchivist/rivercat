use arrayref::array_ref;

pub trait Block {}

// 256-bit block
#[derive(PartialEq)]
pub struct Block256 {
    pub data: [u8; 32],
}

impl Block for Block256 {}

impl core::fmt::Debug for Block256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // write as hex
        write!(f, "Block256:\n {:x?}\n", self.data).unwrap();
        // combine all data into one hex string
        let mut hex = String::new();
        hex.push_str("0x");
        for i in 0..32 {
            hex.push_str(&format!("{:02x}", self.data[i]));
        }
        write!(f, "block-str: {}", hex)
    }
}

impl Default for Block256 {
    fn default() -> Self {
        Self { data: [0; 32] }
    }
}

impl Block256 {
    pub fn new(data: [u8; 32]) -> Self {
        Self { data }
    }
}

impl From<&[u8; 32]> for Block256 {
    fn from(data: &[u8; 32]) -> Self {
        Self { data: *data }
    }
}

impl From<&[u8; 16]> for Block128 {
    fn from(data: &[u8; 16]) -> Self {
        Self { data: *data }
    }
}

impl From<(Block128, Block128)> for Block256 {
    fn from(value: (Block128, Block128)) -> Self {
        // use LHS and RHS to make new 256 block
        let (lhs, rhs) = value;
        let mut data = [0; 32];
        for i in 0..16 {
            data[i] = lhs.data[i];
            data[i + 16] = rhs.data[i];
        }
        Block256 { data }
    }
}

impl From<u128> for Block128 {
    fn from(value: u128) -> Self {
        let mut data = [0; 16];
        for i in 0..16 {
            data[i] = (value >> (i * 8)) as u8;
        }
        Self { data }
    }
}

// 128-bit block
#[derive(Debug, Clone, Copy)]
pub struct Block128 {
    pub data: [u8; 16],
}

impl Block for Block128 {}

impl Default for Block128 {
    fn default() -> Self {
        Self { data: [0; 16] }
    }
}

impl Block128 {
    pub fn new(data: [u8; 16]) -> Self {
        Self { data }
    }

    pub fn wrapping_add(&self, rhs: &Self) -> Block128 {
        let mut result = [0; 16];
        for i in 0..16 {
            result[i] = self.data[i].wrapping_add(rhs.data[i]);
        }
        Self { data: result }
    }
}

impl From<[u8; 16]> for Block128 {
    fn from(data: [u8; 16]) -> Self {
        Self { data }
    }
}

// Split a 256-bit block into LHS and RHS 128-bit blocks
impl From<Block256> for (Block128, Block128) {
    fn from(value: Block256) -> Self {
        let left = array_ref!(value.data, 0, 16);
        let right = array_ref!(value.data, 16, 16);
        let (left_block, right_block) = (Block128::from(left.clone()), Block128::from(right.clone()));
        (left_block, right_block)
    }
}

// Direct BitXor on 128-bit blocks
impl core::ops::BitXor for Block128 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = [0; 16];
        for i in 0..16 {
            result[i] = self.data[i] ^ rhs.data[i];
        }
        Self { data: result }
    }
}

