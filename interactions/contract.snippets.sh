PROJECT="${PWD}"

COLLECTION_ID="EQUISTAR-3f393f"
COLLECTION_ID_HEX="0x$(echo -n ${COLLECTION_ID} | xxd -p -u | tr -d '\n')"

TOKEN_ID="ESTAR-461bab"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

TOKEN_TEST_ID="ANOTHERCOL-60e481"
TOKEN_TEST_ID_HEX="0x$(echo -n ${TOKEN_TEST_ID} | xxd -p -u | tr -d '\n')"

PEM_FILE="/home/edi/Desktop/wallet-estar/wallet-owner.pem"
PROXY=https://gateway.multiversx.com
CHAINID=1
ADDRESS=erd1qqqqqqqqqqqqqpgqq3uzjptflvpythrflnfxry8sf52kuedtwmfs4x6xxz
MY_ADDRESS="erd1szcgm7vq3tmyxfgd4wd2k2emh59az8jq5jjpj9799a0k59u0wmfss4vw3v"

deploy() {
  mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX $TOKEN_ID_HEX || return
}

updateContract() {
  mxpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX $TOKEN_ID_HEX
}

stake() {
  method_name="0x$(echo -n 'stake' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=12000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="MultiESDTNFTTransfer" \
    --arguments $ADDRESS 2 $COLLECTION_ID_HEX 7 1 $COLLECTION_ID_HEX 8 1 $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/stake.json"
}
unStake() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="unStake" \
    --arguments 7 \
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

setRewardPerNft() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=60000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setRewardPerNft" \
    --arguments 50000000000000000000 \
    --send \
    --outfile="${PROJECT}/interactions/logs/unbond.json"
}

setAllowList() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=500000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="setAllowList" \
    --arguments 4188 4182 4112 3860 3853 3726 3725 3705 3148 2986 2865 2838 2833 2732 2707 2565 2501 2436 1894 1844 1810 1794 1722 1587 1439 1351 1332 1260 1239 1223 1210 1055 731 664 496 414 411 332 207 165 136 110 35 14 \
    --send \
    --outfile="${PROJECT}/interactions/logs/unbond.json"
}

fundSystem() {
  method_name="0x$(echo -n 'fundSystem' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $TOKEN_ID_HEX 100000000000000000000 $method_name \
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
    --gas-limit=4000000 \
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
  mxpy --verbose contract query ${ADDRESS} --function="getNftStakedAt" --arguments 5 \
    --proxy=${PROXY}
}

getCollection() {
  mxpy --verbose contract query ${ADDRESS} --function="getCollection" \
    --proxy=${PROXY}
}

getRewardToken() {
  mxpy --verbose contract query ${ADDRESS} --function="getRewardTokenAmount" \
    --proxy=${PROXY}
}

getRewardPerNft() {
  mxpy --verbose contract query ${ADDRESS} --function="getRewardPerNft" \
    --proxy=${PROXY}
}

getRewardToken() {
  mxpy --verbose contract query ${ADDRESS} --function="getRewardToken" \
    --proxy=${PROXY}
}

getRewards() {
  mxpy --verbose contract query ${ADDRESS} --function="getRewards" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}

getUserLastClaim() {
  mxpy --verbose contract query ${ADDRESS} --function="getUserLastClaim" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}

getRewardTokenAmount() {
  mxpy --verbose contract query ${ADDRESS} --function="getRewardTokenAmount" \
    --proxy=${PROXY}
}

getAllowList() {
  mxpy --verbose contract query ${ADDRESS} --function="allowList" \
    --proxy=${PROXY}
}