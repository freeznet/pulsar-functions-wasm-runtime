## autogen

```
cargo install --git https://github.com/freeznet/wit-bindgen --branch pfwasm wit-bindgen-cli

wit-bindgen host wasmtime-rust --export context.wit --rustfmt
```