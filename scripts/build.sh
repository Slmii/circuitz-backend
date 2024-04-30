# Build script canister

# Generate canister candid
cargo test candid -p canister

# Build wasm
cargo build -p canister --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/canister.wasm > target/wasm32-unknown-unknown/release/canister.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/canister.wasm.gz wasm/canister.wasm.gz

####

# Generate nodes candid
cargo test candid -p nodes

# Build wasm
cargo build -p nodes --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/nodes.wasm > target/wasm32-unknown-unknown/release/nodes.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/nodes.wasm.gz wasm/nodes.wasm.gz
