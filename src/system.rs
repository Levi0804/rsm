use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy;
    type Nonce: Copy + CheckedSub + CheckedAdd + Zero + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> 
where 
    T::AccountId: Ord + Clone,
    T::BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy,
    T::Nonce: Copy + CheckedSub + CheckedAdd + Zero + One,
{
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>
}

impl<T: Config> Pallet<T>
where 
    T::AccountId: Ord + Clone,
    T::BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy,
    T::Nonce: Copy + CheckedSub + CheckedAdd + Zero + One,
{
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new()
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce.checked_add(&T::Nonce::one()).unwrap();
        self.nonce.insert(who.clone(),  new_nonce);
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system = super::Pallet::<TestConfig>::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let mut system = super::Pallet::<TestConfig>::new();
        system.inc_nonce(&"alice".to_string());        
        assert_eq!(system.nonce.get("alice").unwrap(), &1);
    }

}