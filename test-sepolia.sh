# Build the project
echo "Building the project..."
cargo build

# These are some examples of live values that you can use to test the publisher
export TOYKEN_ADDRESS=0x185Bb1cca668C474214e934028A3e4BB7A5E6525
export PROVING_BLOCK_NUMBER=6624412
export VOTER_SIGNATURE="8aab07022698deb29dc50189b0f6d8cf14c9164841b98ba1b45b74d4949cff80457e7cfc90449792fc113e19ffcb2006d2e74da6c02cdb739b3e14faa501a4f31b"
export VOTER=0x983564F7c30Da047aC680764dE089D269fc3cbfb
export COUNTER_ADDRESS=0x160beb4C2bb5a64AadDc38fCeA93e42e107FEac7 
export DAO_ADDRESS=0x11E4b66Ea71b3687aA4d09E83A78eA4068890104
export PROPOSAL_ID=0
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
echo "Verifying state..."
COUNTER_VALUE=$((COUNTER_VALUE + 1))
NEW_COUNTER_VALUE=$(cast call --rpc-url ${RPC_URL} ${COUNTER_ADDRESS:?} 'get()(uint256)')
if [ "$NEW_COUNTER_VALUE" != "$COUNTER_VALUE" ]; then
    echo "Counter value is not $COUNTER_VALUE as expected, but $NEW_COUNTER_VALUE."
    exit 1
fi

echo "All operations completed successfully."
