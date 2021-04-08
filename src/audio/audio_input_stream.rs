use crate::ffi::SPXAUDIOSTREAMHANDLE;

/// Abstraction over audio input push & pull streams
pub trait AudioInputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE;
}
