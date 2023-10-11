use crate::block::{Block128, Block256};
use crate::crypto::scheduler::Scheduler;
use super::round_function::RoundFunction256;

pub struct FeistelNet128 {
    left: Block128,
    right: Block128,
    key_i: Block128
}

impl FeistelNet128 {
    pub fn new(data: Block256, key_i: Block128) -> Self {
        let (left, right) = data.into();
        Self { left, right, key_i }
    }

    fn update_state(&mut self, left: Block128, right: Block128) {
        self.left = left;
        self.right = right;
    }

    pub fn encrypt_with<F>(&mut self, r: usize)
    where
        F: RoundFunction256,
    {
        let (mut left, mut right) = (self.left, self.right);

        for _ in 0..r {
            let temp = left;
            left = right ^ F::f(left, self.key_i);
            right = temp;
        }

        self.update_state(left, right);
    }
}

pub struct ScheduledFeistelNet128<S>
where
    S: Scheduler<Block128>,
{
    left: Block128,
    right: Block128,
    key_i: S,
}

impl<S> ScheduledFeistelNet128<S> 
where
    S: crate::crypto::scheduler::Scheduler<Block128>,
{
    pub fn new(data: Block256, key_i: S) -> Self {
        let (left, right) = data.into();
        Self { left, right, key_i }
    }

    fn update_state(&mut self, left: Block128, right: Block128) {
        self.left = left;
        self.right = right;
    }

    pub fn encrypt_with<F>(&mut self, r: usize)
    where
        F: RoundFunction256,
    {
        self.key_i.set_position(0);
        log::debug!("(feistel) encrypting plaintext with {} rounds of Feistel Ciphers", r);
        let (mut left, mut right) = (self.left, self.right);
        for round in 0..r {
            let temp = left;
            left = right ^ F::f(left, self.key_i.get_key_at_position(round));
            right = temp;
        }
        self.update_state(left, right);
    }

    pub fn decrypt_with<F>(&mut self, r: usize)
    where
        F: RoundFunction256,
    {
        self.key_i.set_position(r-1);
        log::debug!("(rivercat) setting iterative rijndael schedule position to {}", r - 1);
        log::debug!("(feistel) decrypting ciphertext");
        let (mut left, mut right) = (self.left, self.right);
        for round in (0..r).rev() {
            let temp = right;
            right = left ^ F::f(right, self.key_i.get_key_at_position(round));
            left = temp;
        }
        self.update_state(left, right);
    }
}

#[cfg(test)]
mod smoke_test {
    use super::*;
    use crate::crypto::{feistel::round_function::NaiveWrappingAdd, scheduler::{KeyExpander, IterativeRijndaelScheduler}};

    #[test]
    fn test_feistel_scheduled() {
        pretty_env_logger::try_init().ok();

        let input = Block256::from(b"hello world hello this is my256!");
        let input_copy = Block256::from(b"hello world hello this is my256!");
        log::debug!("encrypting input: {:x?}", input);
        let key = Block128::from(b"thisiakey128bits");
        log::debug!("using key: {:x?}", key);
        let r = 32;
        log::debug!("over {} rounds", r);

        let key_scheduler = IterativeRijndaelScheduler::new(key);
        log::debug!("using IterativeRijndaelScheduler");

        let mut feistel = ScheduledFeistelNet128::<IterativeRijndaelScheduler>::new(input, key_scheduler);
        let _ = feistel.encrypt_with::<NaiveWrappingAdd>(r);
        let ciphertext = (feistel.left, feistel.right);
        log::debug!("ciphertext after {} rounds: {:x?} {:x?}", r, ciphertext.0, ciphertext.1);

        let _ = feistel.decrypt_with::<NaiveWrappingAdd>(r);
        let recovered = (feistel.left, feistel.right);
        let recovered256 = Block256::from(recovered);
        log::info!("decrypted ciphertext: {:?}", recovered256);

        assert_eq!(input_copy, recovered256);
    }
}
