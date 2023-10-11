use crate::block::Block128; 

// F: B x K --> B
//   where B is the block-width (128 or 256-bit blocks)
//   and K is the width of the round key (128 or 256-bit keys)
#[allow(non_snake_case)]
pub trait RoundFunction256
{
    fn f(input: Block128, K_i: Block128) -> Block128;
}

pub struct NaiveWrappingAdd;

impl RoundFunction256 for NaiveWrappingAdd {
    #[allow(non_snake_case)]
    fn f(input: Block128, K_i: Block128) -> Block128 {
        input.wrapping_add(&K_i)
    }
}

