default: build

all: test

test: test
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l ../target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
