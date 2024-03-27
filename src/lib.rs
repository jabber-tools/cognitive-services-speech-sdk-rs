#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! # cognitive-services-speech-sdk-rs
//!
//! Pure Rust binding for the Microsoft Cognitive Service Speech SDK.
//! Provides thin abstraction around native C API.
//! Heavily inspired by official [Golang library](https://github.com/microsoft/cognitive-services-speech-sdk-go).
//!
//! For more information about Micorost Speech Service see [here](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-sdk?tabs=windows%2Cubuntu%2Cios-xcode%2Cmac-xcode%2Candroid-studio).
pub mod audio;
pub mod common;
pub mod dialog;
pub mod error;
pub mod ffi;
pub mod speech;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
