#![allow(unused)]
use support::Dipatch;
mod balances;
mod system;
mod support;
mod proof_of_existence;

mod types {
    use crate::support;
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Runtime>,
    system: system::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { call, caller })  in block.extrinsics.into_iter().enumerate() { 
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e|
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number,i , e
                )
            );
        }
        Ok(())
    }
}

impl crate::support::Dipatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, runtime_call: Self::Call) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances (call) => {
                // we are dispatching to balances pallet
                self.balances.dispatch(caller, call)?; 
            },
            RuntimeCall::ProofOfExistence(call) => {
                // we are dispatching to proof_of_existence pallet
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice.clone(), 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20 }),
            }
        ]
    };

    runtime.execute_block(block_1).unwrap();

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: "my_document" }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: "bobs document" }), 
            }
        ]
    };

    runtime.execute_block(block_2).unwrap();


    println!("{:#?}", runtime);
}   