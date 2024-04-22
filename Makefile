.PHONY: clean
clean:
	cargo clean

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: test
test:
	cargo test --all --workspace --bins --tests --benches

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets --all-features --tests -- -D warnings


.PHONY: build-token
build-token:
	cd token && cargo build --release --target wasm32-unknown-unknown

.PHONY: build-deployer
build-deployer:
	cd deploy && cargo build --release --target wasm32-unknown-unknown

CONTRACT_DEPLOYER=SCBDHL6YTFK4FUQIWRXPM2HZ6KAA7YECCOK6Y7RTYLTWRNJ2XDHHBH5R
ADMIN_ACCOUNT=GDWYLT6ACXVSPY65UWJ2WRHL45M6HHPMNJWPNDRRCHU2A76TFV5OQBNX
NETWORK_PASSPHRASE='Test SDF Future Network ; October 2022'
RPC_URL='https://rpc-futurenet.stellar.org:443/'

.PHONY: deploy-token
deploy-token:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
        --source $(CONTRACT_DEPLOYER) \
        --rpc-url $(RPC_URL) \
        --network-passphrase $(NETWORK_PASSPHRASE)

.PHONY: initialize
initialize:
	soroban contract invoke \
        --source $(CONTRACT_DEPLOYER) \
        --rpc-url $(RPC_URL) \
        --network-passphrase $(NETWORK_PASSPHRASE) \
		--id $(CONTRACT_ID) \
		-- initialize --admin $(ADMIN_ACCOUNT) --decimal 6 --name "Excellar Token" --symbol "XUSD"

.PHONY: pass-kyc
pass-kyc:
	soroban contract invoke \
        --source $(CONTRACT_ADMIN) \
        --rpc-url $(RPC_URL) \
        --network-passphrase $(NETWORK_PASSPHRASE) \
		--id $(CONTRACT_ID) \
		-- pass_kyc --addr $(ADDR)

.PHONY: mint
mint:
	soroban contract invoke \
        --source $(CONTRACT_ADMIN) \
        --rpc-url $(RPC_URL) \
        --network-passphrase $(NETWORK_PASSPHRASE) \
		--id $(CONTRACT_ID) \
		-- mint --amount $(AMOUNT) --to $(ADDR)

.PHONY: balance
balance:
	soroban contract invoke \
        --source $(CONTRACT_ADMIN) \
        --rpc-url $(RPC_URL) \
        --network-passphrase $(NETWORK_PASSPHRASE) \
		--id $(CONTRACT_ID) \
		-- balance --id $(ADDR)
