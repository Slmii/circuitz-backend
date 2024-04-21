# Build script canister

# Generate candid
cargo test candid -p canister

# Build wasm
cargo build -p canister --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/canister.wasm > target/wasm32-unknown-unknown/release/canister.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/canister.wasm.gz wasm/canister.wasm.gz
