use crate::ffi::SPXAUDIOSTREAMHANDLE;

/// Abstraction over audio input push & pull streams
/// for now we are not supporting pull streams. This
/// is mostly for future use
pub trait AudioInputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE;
}
