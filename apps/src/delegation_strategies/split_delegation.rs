use super::DelegationStrategy;
use crate::Delegation;
use crate::{Asset, HostEvmEnv};
use alloy_primitives::Address;
use alloy_sol_types::sol;
use anyhow::{bail, Result};
use risc0_steel::{host::provider::Provider, Contract, EvmBlockHeader};

sol! {
    /// ERC-20 balance function signature.
    interface DelegateRegistry {
        struct Delegation {
            bytes32 delegate;
            uint256 ratio;
        }
        function getDelegation(string memory context, address account) public view returns (Delegation[] memory delegations, uint256 expirationTimestamp);
    }
}

pub struct SplitDelegation;

impl<P, H> DelegationStrategy<P, H> for SplitDelegation
where
    P: Provider,
    H: EvmBlockHeader,
{
    fn process(
        &self,
        env: &mut HostEvmEnv<P, H>,
        account: Address,
        asset: &Asset,
        additional_data: Vec<u8>,
    ) -> Result<Vec<Delegation>> {
        let context = "0x";
        // Ensure the length of the input bytes is a multiple of 20
        if additional_data.len() % 20 != 0 {
            bail!("Input byte vector is not a valid length for Address conversion");
        }

        // Collect chunks of 20 bytes and convert them into `Address`
        let delegations: Vec<Address> = additional_data
            .chunks_exact(20) // Split the input bytes into chunks of 20
            .map(|chunk| Address::from_slice(chunk)) // Convert each chunk into an `Address`
            .collect();

        // Confirm the delegations are valid and get each ratio
        let mut delegations_contract = Contract::preflight(asset.token, env);

        let account_delegates: Vec<Option<Delegation>> = delegations
            .iter()
            .map(|potential_delegate| {
                let potential_delegate_delegations_call = DelegateRegistry::getDelegationCall {
                    context: context.to_string(),
                    account: *potential_delegate,
                };
                let potential_delegate_delegations = delegations_contract
                    .call_builder(&potential_delegate_delegations_call)
                    .call()
                    .unwrap();

                // Find the matching delegation for the account and return a Some(Delegation) if valid
                potential_delegate_delegations
                    .delegations
                    .iter()
                    .find(|d| compare_bytes32_to_address(d.delegate, account))
                    .map(|d| Delegation {
                        delegate: *potential_delegate,
                        ratio: d.ratio,
                    })
            })
            .collect();

        if account_delegates.iter().any(|d| d.is_none()) {
            bail!("One or more delegations are invalid");
        } else {
            Ok(account_delegates.into_iter().map(|d| d.unwrap()).collect())
        }
    }
}

fn compare_bytes32_to_address(bytes32: alloy_primitives::FixedBytes<32>, address: Address) -> bool {
    // Extract the last 20 bytes of the bytes32 (rightmost part of the bytes32)
    let bytes = bytes32.as_slice();
    let last_20_bytes = &bytes[12..]; // From index 12 to the end (20 bytes)

    // Compare the last 20 bytes to the address bytes
    last_20_bytes == address
}
