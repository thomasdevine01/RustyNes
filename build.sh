rustc +nightly --target wasm32-unknown-unknown -O nes.rs

python3 -m http.server