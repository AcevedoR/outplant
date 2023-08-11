# unnamed-game
a simple text game POC

## requirements
- rust
- cargo
- do Yem and WASM setup https://yew.rs/docs/getting-started/introduction

## start
```
trunk serve --features wasm-logging
```

## test
```
cargo test
```

to display logs and full stacktrace:
```
export RUST_BACKTRACE=1 && cargo test -- --nocapture
```
