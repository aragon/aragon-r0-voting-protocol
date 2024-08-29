use super::ProtocolStrategy;
use crate::Asset;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use risc0_steel::{Contract, EvmEnv};

sol! {
    /// ERC-20 balance function signature.
    interface IERC20Votes {
        function getPastVotes(address account, uint256 blockNumber) external view returns (uint);
        function getPastTotalSupply(uint256 timepoint) external view returns (uint);
    }
}

pub struct GetPastVotes;
impl ProtocolStrategy for GetPastVotes {
    fn process(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        account: Address,
        asset: &Asset,
    ) -> U256 {
        let block_number = env.block_commitment().blockNumber;
        let asset_contract = Contract::new(asset.token, env);
        let balance_call = IERC20Votes::getPastVotesCall {
            account,
            blockNumber: block_number,
        };
        let balance = asset_contract.call_builder(&balance_call).call();
        U256::from(balance._0)
    }

    fn get_supply(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        asset: &Asset,
    ) -> U256 {
        let block_number = env.block_commitment().blockNumber;
        let asset_contract = Contract::new(asset.token, env);
        let supply_call = IERC20Votes::getPastTotalSupplyCall {
            timepoint: block_number,
        };
        let supply = asset_contract.call_builder(&supply_call).call();
        U256::from(supply._0)
    }
}
