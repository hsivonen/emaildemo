# emaildemo

This is a demo for a potential new algorithm for validating HTML `input type=email` and turning the value into a submittable form and into a presentable form. (It's unclear if the presentable form should appear in a browser or be exclusively computed by a server by running `ToUnicode` on the domain.)

## Building

````sh
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/emaildemo.wasm --out-dir . --target web --no-typescript
````

## Files needed on server

````
index.html
emaildemo.js
emaildemo_bg.wasm
target/wasm32-unknown-unknown/release/emaildemo.wasm
````

## License

CC0