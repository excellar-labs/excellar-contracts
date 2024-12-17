# excellar

Stellar excellar is a decentralized lending platform built on the Stellar Network. 
It allows users to tokenize, lend and borrow money market assets.

## Getting Started
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked --version 0.8.0 soroban-cli
```

## Test
```bash
cargo test
```
## Build and deploy
```bash
cargo build --target wasm32-unknown-unknown --release
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/excellar.wasm \
    --source S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

### Deployment

#### Deploy and intiialize the test contract
```bash
make build-token
make deploy-token
make initialize CONTRACT_ID=CB4VMGDWQLUXO7HCXSRAOLNJKFBQQT2K62QT2FY24ZRBVCLRQM2TVUSY
```

### Testing the functionality

#### Invoke the USDC contract and mint tokens
```bash

 make pass_kyc CONTRACT_ID=CB4VMGDWQLUXO7HCXSRAOLNJKFBQQT2K62QT2FY24ZRBVCLRQM2TVUSY ADDR=GD4WKNE22Q3ZAOEBIGWNRCXDYOKC6AHHWYHI4ECMXFY2BKUSMZL4YN7U
 make mint CONTRACT_ID=CB4VMGDWQLUXO7HCXSRAOLNJKFBQQT2K62QT2FY24ZRBVCLRQM2TVUSY ADDR=GD4WKNE22Q3ZAOEBIGWNRCXDYOKC6AHHWYHI4ECMXFY2BKUSMZL4YN7U AMOUNT=1000000
```

#### Withdraw own deposit from tokenizer contract
```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account SAVQKTSXS3T2VNXQRESDPWEAYT5HCSA6GRXPCGUF6HZDM2EOLGYDHFY6 \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CACLTFEE37H73JK7MMOCRSS2MOGR7DX7OUDRKAPDFHDXQHEGQZDHKH7U \
    -- withdraw --to GBGXBIEMYC7F2OVWVXKNJVYXSRUS4BXF57L5IZWHMDJIPTFPP5Z7TNIP --share-amount 20
```
##### Withdraw all as admin

```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account SAVQKTSXS3T2VNXQRESDPWEAYT5HCSA6GRXPCGUF6HZDM2EOLGYDHFY6 \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CACLTFEE37H73JK7MMOCRSS2MOGR7DX7OUDRKAPDFHDXQHEGQZDHKH7U \
    -- withdraw_admin --to GDOJ6OUGJYOQL2SQ52A2R33KOYHJMJ2DCLZZEYUXUKJBB3CSIO5ZKKQ5 --usdc-amount 20
```

##### Checking a trust line
```bash
  echo '{"type": "trustline", "account_id": "GDC3HLVFTCK57SDEYQGUYGAKOE5YJHHTVQVZ5KQ5OJZG6C46TODAFGWY", "asset": "XUSD:GBUB6OYV54IR5Y4UWVGYADFFTXLT6VW7U3ZPVEM7SRU3CRP75KIDWVSU"}' | stellar xdr encode --type LedgerKey --input json --output single-base64
  
```