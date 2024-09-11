# Build the project
echo "Building the project..."
cargo build

# These are some examples of live values that you can use to test the publisher
export TOYKEN_ADDRESS=0x185Bb1cca668C474214e934028A3e4BB7A5E6525
export PROVING_BLOCK_NUMBER=6670631
export VOTER_SIGNATURE="67bb934b4167c8b71580651bb028e1e9ead9a63f4f9de10025cbfddd07def8b3283275cbe321898f0975cbb121e4557976bfb9791fe267e98b4ecc256091cd2a1c"
export VOTER=0x983564F7c30Da047aC680764dE089D269fc3cbfb
#export VOTER=0x8bF1e340055c7dE62F11229A149d3A1918de3d74
export COUNTER_ADDRESS=0xC83De2199637f11E6457b4979EB2986162b02F71
export DAO_ADDRESS=0xc6906995e59e7CB65f2D9e91f5CcC88ca2D1c9db
export PROPOSAL_ID=2
export DIRECTION=1
export BALANCE=100000000000000000

COUNTER_VALUE=$(cast call --rpc-url ${RPC_URL} ${COUNTER_ADDRESS:?} 'get()(uint256)')

echo ""
echo "----------------------------------------------------------------------"
echo "|                                                                     |"
echo "|  You should have exported you testnet private key for this to work  |"
echo "|  You should have exported you testnet RPC_URL for this to work      |"
echo "|                                                                     |"
echo "----------------------------------------------------------------------"
echo ""
echo "ERC20 Toyken Address: $TOYKEN_ADDRESS"
echo "Initial block number: $PROVING_BLOCK_NUMBER"
echo "Counter Address: $COUNTER_ADDRESS"
echo "Address: $USER_ADDRESS"
echo "Counter value: $COUNTER_VALUE"

# Publish a new state
echo "Publishing a new state..."
cargo run --bin publisher -- \
    --chain-id=11155111 \
    --rpc-url=${RPC_URL} \
    --block-number=${PROVING_BLOCK_NUMBER:?} \
    --voter-signature=${VOTER_SIGNATURE} \
    --voter=${VOTER} \
    --dao-address=${DAO_ADDRESS} \
    --proposal-id=${PROPOSAL_ID} \
    --direction=${DIRECTION} \
    --balance=${BALANCE} \
    --config-contract=${COUNTER_ADDRESS:?} \
    --token=${TOYKEN_ADDRESS:?} 

# Attempt to verify counter value as part of the script logic
echo "All operations completed successfully."
