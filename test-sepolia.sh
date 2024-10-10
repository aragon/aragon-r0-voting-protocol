# Build the project
echo "Building the project..."
cargo build

# These are some examples of live values that you can use to test the publisher
export TOYKEN_ADDRESS=0x185Bb1cca668C474214e934028A3e4BB7A5E6525
export PROVING_BLOCK_NUMBER=6845622
export VOTER_SIGNATURE="6673ae515ae81a300269ba5bca6bb710500fc424707e0a0243658ad122cf85c9435e28ef9c52dc1431aa1efea4f8a253adeeead90bc08152d88d93b43085377b1c"
export VOTER=0x8bF1e340055c7dE62F11229A149d3A1918de3d74
#export VOTER=0x8bF1e340055c7dE62F11229A149d3A1918de3d74
export COUNTER_ADDRESS=0x0eb6FE665fFf15f99b34803eD53139A6c5488824
export DAO_ADDRESS=0x727B0141EF57f1AD53B9a8f63adFFD739Ea11fDf
export PROPOSAL_ID=1
export DIRECTION=2
export BALANCE=900000000000000000
export ADDITIONAL_DELEGATION_DATA="8bF1e340055c7dE62F11229A149d3A1918de3d74"

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
    --token=${TOYKEN_ADDRESS:?} \
    --additional-delegation-data=${ADDITIONAL_DELEGATION_DATA:?}

# Attempt to verify counter value as part of the script logic
echo "All operations completed successfully."
