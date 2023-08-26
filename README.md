# Outplant

A simple text game POC.

## requirements

- rust
- cargo
- do Yem and WASM setup https://yew.rs/docs/getting-started/introduction

## start

```
trunk serve --features wasm-logging
```

## format

```
cargo fmt
```

## test

```
cargo test
```

To display logs and full stacktrace:

```
export RUST_BACKTRACE=1 && cargo test -- --nocapture
```
