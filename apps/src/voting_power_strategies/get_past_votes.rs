use super::VotingPowerStrategy;
use crate::{Asset, HostEvmEnv};
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use risc0_steel::{host::provider::Provider, Contract, EvmBlockHeader};

sol! {
    /// ERC-20 balance function signature.
    interface IERC20Votes {
        function getPastVotes(address account, uint256 blockNumber) external view returns (uint);
        function getPastTotalSupply(uint256 timepoint) external view returns (uint);
    }
}

pub struct GetPastVotes;

impl<P, H> VotingPowerStrategy<P, H> for GetPastVotes
where
    P: Provider,
    H: EvmBlockHeader,
{
    fn process(&self, env: &mut HostEvmEnv<P, H>, account: Address, asset: &Asset) -> U256 {
        let block_number = env.block_commitment().blockNumber;
        let mut asset_contract = Contract::preflight(asset.contract, env);
        let balance_call = IERC20Votes::getPastVotesCall {
            account,
            blockNumber: block_number,
        };
        let balance = asset_contract.call_builder(&balance_call).call().unwrap();
        U256::from(balance._0)
    }
}
