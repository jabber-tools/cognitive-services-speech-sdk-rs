# cognitive-services-speech-sdk-rs examples

---
[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](../../graphs/commit-activity)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Build Status](https://www.travis-ci.com/jabber-tools/cognitive-services-speech-sdk-rs.svg?branch=main)](https://www.travis-ci.com/github/jabber-tools/cognitive-services-speech-sdk-rs)

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
