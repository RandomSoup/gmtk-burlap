# Burlap: Wasm Edition

## Building

Can be used like normal with `cargo run/build/install/etc` and to make the web version use:
```
RUSTFLAGS="-C link-args=--no-entry" cross build --release --lib --target wasm32-unknown-emscripten --features=wasm --no-default-features
```


## Limits

No file IO or importing, the file must be `src/main.sk`

## TODO

Don't waste time lexing/parsing/etc, just run bytecode.