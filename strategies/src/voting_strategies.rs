use crate::Asset;
use alloy_primitives::{Address, U256};
use risc0_steel::EvmEnv;

pub trait ProtocolStrategy {
    fn process(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        account: Address,
        asset: &Asset,
    ) -> U256;

    fn get_supply(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        asset: &Asset,
    ) -> U256;
}

mod balance_of;
mod get_past_votes;

pub use balance_of::BalanceOf;
pub use get_past_votes::GetPastVotes;
