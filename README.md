# Build steps
1. cargo build --target wasm32-unknown-unknown --release
2. wasm2luau ./target/wasm32-unknown-unknown/release/lua_wasm.wasm > ./oof.luau
