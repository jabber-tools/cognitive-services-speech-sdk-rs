# cognitive-services-speech-sdk-rs

---
[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](../../graphs/commit-activity)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Build Status](https://www.travis-ci.com/jabber-tools/cognitive-services-speech-sdk-rs.svg?branch=main)](https://www.travis-ci.com/github/jabber-tools/cognitive-services-speech-sdk-rs)

Attempt to write Microsoft cognitive speech services SDK in Rust. Inspired by [Go library](https://github.com/microsoft/cognitive-services-speech-sdk-go) older [Rust implementation](https://github.com/masayoshi-louis/microsoft-speech-rs). Initially only subset of functionality will be implemented in order to support active speech recognition use case. The design goal is skip most of abstraction seen in Rust library and use more straightforward approach (basically creating very raw and minimalistic wrappers around underlying C API). This is the approach used in above mentioned Go library.

## Build prerequisites

This library is utilizing Microsoft Speech SDK. Details can be found here [here](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/quickstarts/setup-platform?tabs=dotnet%2Cwindows%2Cjre%2Cbrowser&pivots=programming-language-go)

```
sudo apt-get update 
sudo apt-get install clang build-essential libssl1.0.0 libasound2 wget
```