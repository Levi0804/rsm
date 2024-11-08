use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::{support::DispatchResult, system};

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(&claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                return Ok(());
            }
        }
    }

    pub fn remoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}

pub enum Call<T: Config> {
    CreateClaim {claim: T::Content },
    RemoveClaim {claim: T::Content },
}

impl <T: Config> crate::support::Dipatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RemoveClaim { claim } => self.remoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();

        let _ = poe.create_claim("alice", "my_document");

        assert_eq!(poe.get_claim(&"my_document"), Some(&"alice"));

        let res= poe.remoke_claim(&"bob", "my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        let res = poe.create_claim(&"bob", "my_document");
        assert_eq!(res, Err("Claim already exists"));

        let res = poe.remoke_claim("alice", "non existant");

        assert_eq!(res, Err("Claim does not exist"));

        let res = poe.remoke_claim("alice", "my_document");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claim(&"my_document"), None);

    }

}
