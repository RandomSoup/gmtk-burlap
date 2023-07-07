cargo build --release --target wasm32-unknown-emscripten --features=wasm --no-default-features
cp target/wasm32-unknown-emscripten/release/deps/burlap.data pkg/burlap.data
cp target/wasm32-unknown-emscripten/release/burlap.wasm pkg/burlap.wasm
cp target/wasm32-unknown-emscripten/release/burlap.wasm.map pkg/burlap.wasm.map
cp target/wasm32-unknown-emscripten/release/burlap.d pkg/burlap.d
cp target/wasm32-unknown-emscripten/release/burlap.js pkg/burlap.js