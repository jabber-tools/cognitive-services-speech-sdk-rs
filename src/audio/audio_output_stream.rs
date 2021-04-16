use crate::ffi::SPXAUDIOSTREAMHANDLE;

/// Abstraction over audio output push & pull streams.
/// Enables transparent handling of both pull & push
/// output audio streams by respective functions.
pub trait AudioOutputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE;
}
