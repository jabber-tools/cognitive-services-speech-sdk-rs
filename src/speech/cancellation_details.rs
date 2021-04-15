use crate::common::{CancellationErrorCode, CancellationReason};
use crate::error::{convert_err, Result};
use crate::ffi::{synth_result_get_canceled_error_code, synth_result_get_reason_canceled};
use crate::speech::SpeechSynthesisResult;
use std::mem::MaybeUninit;
use std::os::raw::c_uint;

/// CancellationDetails contains detailed information about why a result was canceled.
/// Added in version 1.17.0
#[derive(Debug)]
pub struct CancellationDetails {
    pub reason: CancellationReason,
    pub error_code: CancellationErrorCode,
}

impl CancellationDetails {
    pub fn from_speech_synthesis_result(
        speech_synthesis_result: SpeechSynthesisResult,
    ) -> Result<Self> {
        unsafe {
            let mut reason: c_uint = MaybeUninit::uninit().assume_init();
            let mut ret = synth_result_get_reason_canceled(
                speech_synthesis_result.handle.inner(),
                &mut reason,
            );
            convert_err(
                ret,
                "CancellationDetails::from_speech_synthesis_result(reason) error",
            )?;

            let mut error_code: c_uint = MaybeUninit::uninit().assume_init();
            ret = synth_result_get_canceled_error_code(
                speech_synthesis_result.handle.inner(),
                &mut error_code,
            );
            convert_err(
                ret,
                "CancellationDetails::from_speech_synthesis_result(error_code) error",
            )?;

            Ok(CancellationDetails {
                reason: CancellationReason::from_u32(reason),
                error_code: CancellationErrorCode::from_u32(error_code),
            })
        }
    }
}
