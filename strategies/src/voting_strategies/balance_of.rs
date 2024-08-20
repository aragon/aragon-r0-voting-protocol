use super::ProtocolStrategy;
use crate::Asset;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use risc0_steel::{Contract, EvmEnv};

sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
}

pub struct BalanceOf;
impl ProtocolStrategy for BalanceOf {
    fn process(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        account: Address,
        asset: &Asset,
    ) -> U256 {
        let asset_contract = Contract::new(asset.token, env);
        let balance_call = IERC20::balanceOfCall { account };
        let balance = asset_contract.call_builder(&balance_call).call();
        U256::from(balance._0)
    }
}
