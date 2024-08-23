# Build the project
echo "Building the project..."
cargo build

# These are some examples of live values that you can use to test the publisher
export TOYKEN_ADDRESS=0x185Bb1cca668C474214e934028A3e4BB7A5E6525
export PROVING_BLOCK_NUMBER=6555792
export VOTER_SIGNATURE="2473ededf4c6205dd5fc9e38590c0e55b1b35493d74082457943f374a810ceb43cb4f31997ba6b43d98a9bcd11e159578ea21e785beac30523f4f491656f550f1b"
export VOTER=0x983564F7c30Da047aC680764dE089D269fc3cbfb
export COUNTER_ADDRESS=0x175415c859C565b96237bf8E8850410fd83A447B
export DAO_ADDRESS=0x175415c859C565b96237bf8E8850410fd83A447B
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
