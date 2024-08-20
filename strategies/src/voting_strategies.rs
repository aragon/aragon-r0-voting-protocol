use alloy_primitives::U256;
use risc0_steel::EvmEnv;

pub trait ProtocolStrategy {
    fn process(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        left: u64,
        right: u64,
    ) -> U256;
}

mod balance_of;
mod get_past_votes;

pub use balance_of::BalanceOf;
pub use get_past_votes::GetPastVotes;
