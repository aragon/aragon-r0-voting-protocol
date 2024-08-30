use crate::Asset;
use alloy_primitives::{Address, U256};
use risc0_steel::EvmEnv;

pub trait ProtocolExecutionStrategy {
    fn proof_execution(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        account: Address,
        asset: &Asset,
    ) -> bool;
}

mod majority_voting;

pub use majority_voting::MajorityVoting;
