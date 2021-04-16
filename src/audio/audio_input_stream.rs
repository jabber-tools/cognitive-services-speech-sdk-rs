use crate::ffi::SPXAUDIOSTREAMHANDLE;

/// Abstraction over audio input push & pull streams.
/// Enables transparent handling of both pull & push
/// input audio streams by respective functions.
pub trait AudioInputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE;
}
