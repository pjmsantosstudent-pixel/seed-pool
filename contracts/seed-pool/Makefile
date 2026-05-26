build:
	soroban contract build

test:
	cargo test

deploy:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/seed_pool.wasm \
		--source $(ADMIN_SECRET) \
		--network testnet