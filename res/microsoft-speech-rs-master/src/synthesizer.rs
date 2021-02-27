use std::borrow::Borrow;
use std::ffi::CString;
use std::sync::Arc;

use num::FromPrimitive;

use crate::{
    AsyncResultHandle,
    CancellationErrorCode,
    CancellationReason,
    convert_err,
    FfiObject,
    FromHandle,
    ResultHandleSupport,
    SmartHandle,
    SpeechConfig,
    SpxError,
    SPXHANDLE_INVALID,
};
use crate::async_handle::AsyncStart;
use crate::audio::AudioConfig;
use crate::ResultReason;
use crate::speech_api::*;

type SpeakAsyncFn = unsafe extern "C" fn(
    SPXSYNTHHANDLE,
    *const ::std::os::raw::c_char,
    u32,
    *mut SPXASYNCHANDLE,
) -> SPXHR;

pub struct SpeechSynthesizer {
    handle: Arc<SmartHandle<SPXSYNTHHANDLE>>,
}

impl SpeechSynthesizer {
    pub fn from_config(config: impl Borrow<SpeechConfig>) -> Result<SpeechSynthesizer, SpxError> {
        let mut handle = SPXHANDLE_INVALID;
        let audio_config = AudioConfig::output_from_default_speaker()?;
        unsafe {
            convert_err(
                synthesizer_create_speech_synthesizer_from_config(
                    &mut handle,
                    config.borrow().get_handle(),
                    audio_config.get_handle(),
                )
            )?;
        }
        Ok(SpeechSynthesizer {
            handle: Arc::new(SmartHandle::create(
                "SpeechSynthesizer",
                handle,
                synthesizer_handle_release,
            ))
        })
    }

    pub fn speak_text_async(&self, text: impl AsRef<str>)
                            -> Result<AsyncResultHandle<SpeakAsyncStart, SpeechSynthesisResult>, SpxError> {
        self.speak_async(text, synthesizer_speak_text_async)
    }

    pub fn speak_ssml_async(&self, text: impl AsRef<str>)
                            -> Result<AsyncResultHandle<SpeakAsyncStart, SpeechSynthesisResult>, SpxError> {
        self.speak_async(text, synthesizer_speak_ssml_async)
    }

    #[inline]
    fn speak_async(&self, text: impl AsRef<str>, f: SpeakAsyncFn)
                   -> Result<AsyncResultHandle<SpeakAsyncStart, SpeechSynthesisResult>, SpxError> {
        let text_len = text.as_ref().len();
        let c_str = CString::new(text.as_ref())?;
        let async_start = SpeakAsyncStart {
            handle: self.handle.clone(),
            f,
            text: c_str,
            text_len,
        };
        AsyncResultHandle::create(
            async_start,
            synthesizer_async_handle_release,
        )
    }
}

pub struct SpeakAsyncStart {
    handle: Arc<SmartHandle<SPXSYNTHHANDLE>>,
    f: SpeakAsyncFn,
    text: CString,
    text_len: usize,
}

impl AsyncStart for SpeakAsyncStart {
    fn name() -> &'static str {
        "TextToSpeechAsyncHandle"
    }

    unsafe fn async_start(&self, hasync: &mut SPXASYNCHANDLE) -> SPXHR {
        (self.f)(self.handle.get(), self.text.as_ptr(), self.text_len as u32, hasync)
    }
}

pub struct SpeechSynthesisResult {
    handle: SmartHandle<SPXRESULTHANDLE>,
}

impl SpeechSynthesisResult {
    pub fn reason(&self) -> Result<ResultReason, SpxError> {
        let code = crate::spx_populate(self.get_handle(), synth_result_get_reason)?;
        return Ok(ResultReason::from_u32(code).expect("unknown reason"));
    }

    pub fn audio_length(&self) -> Result<u32, SpxError> {
        let mut len = 0;
        unsafe {
            convert_err(synth_result_get_audio_length(self.get_handle(), &mut len))?
        }
        Ok(len)
    }

    pub fn audio_data(&self) -> Result<Vec<u8>, SpxError> {
        let buff = FfiObject::new_uninitialized(self.audio_length()? as usize);
        let mut filled_size = 0;
        unsafe {
            convert_err(synth_result_get_audio_data(
                self.get_handle(),
                buff.ptr,
                buff.size as u32,
                &mut filled_size,
            ))?
        }
        Ok(buff.into_vec(filled_size as usize))
    }

    pub fn cancellation_details(&self) -> Result<SpeechSynthesisCancellationDetails, SpxError> {
        let reason = {
            let code = crate::spx_populate(self.get_handle(), synth_result_get_reason_canceled)?;
            CancellationReason::from_u32(code).expect("unknown reason")
        };
        let err_code = {
            let code = crate::spx_populate(self.get_handle(), synth_result_get_canceled_error_code)?;
            CancellationErrorCode::from_u32(code).expect("unknown code")
        };
        Ok(SpeechSynthesisCancellationDetails {
            reason,
            err_code,
        })
    }

    #[inline(always)]
    pub fn get_handle(&self) -> SPXRESULTHANDLE {
        self.handle.get()
    }
}

impl FromHandle<SPXRESULTHANDLE, SpxError> for SpeechSynthesisResult {
    fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self, SpxError> {
        Ok(SpeechSynthesisResult {
            handle: SmartHandle::create(
                "SpeechSynthesisResult",
                handle,
                SpeechSynthesisResult::release_fn(),
            )
        })
    }
}

impl ResultHandleSupport for SpeechSynthesisResult {
    fn async_wait_fn() -> unsafe extern "C" fn(SPXASYNCHANDLE, u32, *mut SPXRESULTHANDLE) -> SPXHR {
        synthesizer_speak_async_wait_for
    }

    fn release_fn() -> unsafe extern "C" fn(SPXRESULTHANDLE) -> SPXHR {
        synthesizer_result_handle_release
    }
}
unsafe impl Sync for SpeechSynthesisResult {}

unsafe impl Send for SpeechSynthesisResult {}

#[derive(Debug)]
pub struct SpeechSynthesisCancellationDetails {
    pub reason: CancellationReason,
    pub err_code: CancellationErrorCode,
}
