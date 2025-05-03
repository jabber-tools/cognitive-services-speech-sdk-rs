use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    speech_synthesis_request_create, speech_synthesis_request_finish,
    speech_synthesis_request_get_property_bag, speech_synthesis_request_release,
    speech_synthesis_request_send_text_piece, SmartHandle, SPXPROPERTYBAGHANDLE, SPXREQUESTHANDLE,
};
use std::ffi::CString;
use std::mem::MaybeUninit;

/// Represents an input stream for speech synthesis request.
#[derive(Debug)]
pub struct SpeechSynthesisRequest {
    pub handle: SmartHandle<SPXREQUESTHANDLE>,
    pub properties: PropertyCollection,
}

unsafe impl Sync for SpeechSynthesisRequest {}

impl SpeechSynthesisRequest {
    /// Creates a speech synthesis request, with text streaming is enabled.
    pub fn new_text_streaming_request() -> Result<Self> {
        unsafe {
            let mut request_handle: MaybeUninit<SPXREQUESTHANDLE> = MaybeUninit::uninit();
            let ret = speech_synthesis_request_create(
                true,
                false,
                std::ptr::null(),
                0,
                request_handle.as_mut_ptr(),
            );
            convert_err(ret, "Failed to create speech synthesis request handle")?;

            let mut prop_bag_handle: MaybeUninit<SPXPROPERTYBAGHANDLE> = MaybeUninit::uninit();
            let ret = speech_synthesis_request_get_property_bag(
                request_handle.assume_init(),
                prop_bag_handle.as_mut_ptr(),
            );
            convert_err(ret, "Failed to get speech synthesis request property bag")?;

            let request = Self {
                handle: SmartHandle::create(
                    "SpeechSynthesisRequest",
                    request_handle.assume_init(),
                    speech_synthesis_request_release,
                ),
                properties: PropertyCollection::from_handle(prop_bag_handle.assume_init()),
            };

            Ok(request)
        }
    }

    /// Send a piece of text to the speech synthesis service to be synthesized, used in text streaming mode.
    pub fn send_text_piece<S: AsRef<str>>(&self, text: S) -> Result<()> {
        let c_text = CString::new(text.as_ref())?;
        let text_len = c_text.as_bytes().len();
        unsafe {
            let ret = speech_synthesis_request_send_text_piece(
                self.handle.inner(),
                c_text.as_ptr(),
                text_len as u32,
            );
            convert_err(ret, "Failed to send text piece")
        }
    }

    /// Finish the text input, used in text streaming mode.
    pub fn finish_input(&self) -> Result<()> {
        unsafe {
            let ret = speech_synthesis_request_finish(self.handle.inner());
            convert_err(ret, "Failed to finish input")
        }
    }
}
