# Build the project
echo "Building the project..."
cargo build

# These are some examples of live values that you can use to test the publisher
export TOYKEN_ADDRESS=0x185Bb1cca668C474214e934028A3e4BB7A5E6525
export PROVING_BLOCK_NUMBER=6503820
export COUNTER_ADDRESS=0x185Bb1cca668C474214e934028A3e4BB7A5E6525
export USER_ADDRESS=0x8bF1e340055c7dE62F11229A149d3A1918de3d74
export COUNTER_ADDRESS=0x421A1Fa1D3b6e8531dd2b2D9e981f5537CB68B24

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
    --contract=${COUNTER_ADDRESS:?} \
    --token=${TOYKEN_ADDRESS:?} \
    --account=${USER_ADDRESS}

# Attempt to verify counter value as part of the script logic
echo "Verifying state..."
COUNTER_VALUE=$((COUNTER_VALUE + 1))
NEW_COUNTER_VALUE=$(cast call --rpc-url ${RPC_URL} ${COUNTER_ADDRESS:?} 'get()(uint256)')
if [ "$NEW_COUNTER_VALUE" != "$COUNTER_VALUE" ]; then
    echo "Counter value is not $COUNTER_VALUE as expected, but $NEW_COUNTER_VALUE."
    exit 1
fi

echo "All operations completed successfully."
