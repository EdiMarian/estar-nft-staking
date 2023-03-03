PROJECT="${PWD}"

TOKEN_ID="EQUISTAR-3f393f"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

TOKEN_ESTAR_ID="ESTAR-461bab"
TOKEN_ESTAR_ID_HEX="0x$(echo -n ${TOKEN_ESTAR_ID} | xxd -p -u | tr -d '\n')"

TOKEN_TEST_ID="ANOTHERCOL-60e481"
TOKEN_TEST_ID_HEX="0x$(echo -n ${TOKEN_TEST_ID} | xxd -p -u | tr -d '\n')"

PEM_FILE="/home/edi/Desktop/wallet-estar/wallet-owner.pem"
PROXY=https://gateway.multiversx.com
CHAINID=1
ADDRESS=erd1qqqqqqqqqqqqqpgq3nnaee50skd7l2c3m7vr7wf8ruv7470mwmfs5d0tll
MY_ADDRESS="erd1szcgm7vq3tmyxfgd4wd2k2emh59az8jq5jjpj9799a0k59u0wmfss4vw3v"

deploy() {
  mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $TOKEN_ID_HEX || return
}

updateContract() {
  mxpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $TOKEN_ID_HEX
}

stake() {
  method_name="0x$(echo -n 'stake' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=60000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="MultiESDTNFTTransfer" \
    --arguments $ADDRESS 1 $TOKEN_ID_HEX 1819 1 $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/stake.json"
}

unStake() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="unStake" \
    --arguments $TOKEN_ID_HEX 1819  \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

togglePause() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=60000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="togglePause" \
    --send \
    --outfile="${PROJECT}/interactions/logs/stake.json"
}

# setNftRarity() {
#   mxpy --verbose contract call ${ADDRESS} --recall-nonce \
#     --pem=${PEM_FILE} \
#     --gas-limit=60000000 \
#     --proxy=${PROXY} --chain=${CHAINID} \
#     --function="setNftRarity" \
#     --arguments 1819 6 \
#     --send \
#     --outfile="${PROJECT}/interactions/logs/unbond.json"
# }

fundSystem() {
  method_name="0x$(echo -n 'fundSystem' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $TOKEN_ESTAR_ID_HEX 100000000000000000000000 $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

withdrawFunds() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="withdrawFunds" \
    --arguments 98142999999999999999124 \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

claimRewards() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="claimRewards" \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

getUsersStaked() {
  mxpy --verbose contract query ${ADDRESS} --function="getUsersStaked" \
    --proxy=${PROXY}
}

getPause() {
  mxpy --verbose contract query ${ADDRESS} --function="getPause" \
    --proxy=${PROXY}
}

getNftsStaked() {
  mxpy --verbose contract query ${ADDRESS} --function="getNftsStaked" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}

getNftStakedAt() {
  mxpy --verbose contract query ${ADDRESS} --function="getNftStakedAt" --arguments 1 \
    --proxy=${PROXY}
}

getToken() {
  mxpy --verbose contract query ${ADDRESS} --function="getToken" \
    --proxy=${PROXY}
}

getNftRarity() {
  mxpy --verbose contract query ${ADDRESS} --function="getNftRarity" --arguments 10009 \
    --proxy=${PROXY}
}

getRewards() {
  mxpy --verbose contract query ${ADDRESS} --function="getRewards" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}

getTokenAmount() {
  mxpy --verbose contract query ${ADDRESS} --function="getTokenAmount" \
    --proxy=${PROXY}
}