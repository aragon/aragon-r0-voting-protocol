use alloy_primitives::U256;
use risc0_steel::EvmEnv;

pub trait ProtocolExecutionStrategy {
    fn proof_execution(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        total_supply: U256,
        tally: [U256; 3],
    ) -> bool;
}

mod majority_voting;

pub use majority_voting::MajorityVoting;
