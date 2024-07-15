# `NCEXE` - Curses based executable display

Supports multiple (Linux, MacOS) executable formats

## Build

```shell
cargo build
```

## Run

```shell
cargo run <exe files>
target/debug/ncexe <exe files>
```

## Examples

```shell
cargo run --example <example.rs>
```

## Test

```shell
cargo test -- --test-threads=1
```

Note: the `--test-threads=1` specification is required or curses doesn't open the windows correctly.