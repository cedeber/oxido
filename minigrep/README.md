# Minigrep

## WebAssembly, WASI and Wasmtime/Wasmer.io

- `rustup target add wasm32-wasi`
- `cargo build --release --target wasm32-wasi`

- `wasmtime run --dir=. target/wasm32-wasi/release/minigrep.wasm en poem.txt`
- `wasmer run --dir=. target/wasm32-wasi/release/minigrep.wasm en poem.txt`
