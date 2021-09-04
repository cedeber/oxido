# Simple Web

The simplest example of communication between WASM and JavaScript.
It uses an "ES module" Worker (Chrome >= 80, Webkit >= 15)

## Build WebAssembly

```sh
cargo watch -w src -- wasm-pack build --target web
wasm-pack build --release --target web
```
