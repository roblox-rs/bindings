# Build steps
1. cargo build --target wasm32-unknown-unknown --release --example test
2. wasm2luau ./target/wasm32-unknown-unknown/release/examples/test.wasm > ./roblox/wasm.luau
3. rojo build --output build.rbxl
