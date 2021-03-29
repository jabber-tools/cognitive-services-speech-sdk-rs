use crate::audio::AudioOutputStream;
use crate::error::Result;
use crate::ffi::{SmartHandle, SPXAUDIOSTREAMHANDLE};

/// not yet implemented, just placeholder
#[derive(Debug)]
pub struct PullAudioOutputStream {
    pub handle: SmartHandle<SPXAUDIOSTREAMHANDLE>,
}

impl AudioOutputStream for PullAudioOutputStream {
    fn get_handle(&self) -> SPXAUDIOSTREAMHANDLE {
        self.handle.inner()
    }
}

impl PullAudioOutputStream {
    pub fn from_handle(_handle: SPXAUDIOSTREAMHANDLE) -> Result<PullAudioOutputStream> {
        unimplemented!();
    }
}
