# dart-sys

[![Crates.io](https://img.shields.io/crates/v/dart-sdk-sys.svg)](https://crates.io/crates/dart-sdk-sys)
[![Documentation](https://docs.rs/dart-sdk-sys/badge.svg)](https://docs.rs/crate/dart-sdk-sys/)
[![Build](https://github.com/DoumanAsh/dart-sys/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/dart-sys/actions?query=workflow%3ARust)

Bindings to dart FFI.

Crate version corresponds to Dart SDK [release](https://github.com/dart-lang/sdk/releases)

## How-to update to new SDK version

1. Update `version` in `Cargo.toml` to be equal to desired version of SDK
2. Run `cargo build --features download-sources,build-bindings`
3. Optionally run `rustfmt src/lib.rs` to make it pretty
4. Commit and publish
