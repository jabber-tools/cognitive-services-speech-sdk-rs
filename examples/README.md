# cognitive-services-speech-sdk-rs examples

---
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Crates.io][crates-badge]][crates-url]
[![docs.rs][rustdoc-badge]][rustdoc-url]
[![CI](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/actions/workflows/github-actions-rust-ci.yml/badge.svg)](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/actions/workflows/github-actions-rust-ci.yml)

[crates-badge]: https://img.shields.io/crates/v/cognitive-services-speech-sdk-rs.svg
[crates-url]: https://crates.io/crates/cognitive-services-speech-sdk-rs
[rustdoc-badge]: https://img.shields.io/badge/docs.rs-1.1.0-green.svg
[rustdoc-url]: https://docs.rs/cognitive-services-speech-sdk-rs


## How to build

```rust
cargo build --examples
```

## How to run

Recognizer examples:

```rust
cargo run --example recognizer
```

Dialog service connector examples:

```rust
cargo run --example dialog
```

Synthesizer examples:

```rust
cargo run --example synthesizer
```

*IMPORTANT*: before running examples proper MS subscription key must be provided. Each example section (recognizer/dialog/synthetizer) contains following line  in *main.rs* file:

```rust
helpers::set_env_vars("/tmp/path/to/subscription/key");
```
*set_env_vars* convenience function loads MS subscription key from text file.
