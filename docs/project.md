# Windows

## Rust Default Settings

cargo 1.90.0 (840b83a10 2025-07-30)

rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.90.0 (1159e78c4 2025-09-14)`

### cargo check

- No modifications
  0.38s
- Modify content in src
  1.87s
- Modify content in crates
  2.61s

### cargo run

- Modify content in src
  13.27s
  13.22s
  13.34s
- Modify content in crates
  11.35s
  11.57s
  11.43s

## Enable Compilation Optimization

- Enable rust nightly
- Enable share-generics
- Enable threads
- Use lld

### cargo check

- No modifications
  0.43s
- Modify content in src
  1.98s
- Modify content in crates
  2.61s

### cargo run

- Modify content in src
  14.06s
  14.10s
  14.07s
- Modify content in crates
  11.57s
  11.69s
  11.62s
