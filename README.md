# unnamed-game
a simple text game POC

## requirements
- rust
- cargo
- do Yem and WASM setup https://yew.rs/docs/getting-started/introduction

## start
```
cargo build
trunk serve
```

## test
```
cargo test --no-default-features
```

to display logs and full stacktrace:
```
export RUST_BACKTRACE=1 && cargo test --no-default-features -- --nocapture
```