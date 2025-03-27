use crate::common::{CancellationErrorCode, CancellationReason, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{synth_result_get_canceled_error_code, synth_result_get_reason_canceled};
use crate::speech::SpeechSynthesisResult;

/// CancellationDetails contains detailed information about why a result was canceled.
/// Added in version 1.17.0
#[derive(Debug)]
pub struct CancellationDetails {
    pub reason: CancellationReason,
    pub error_code: CancellationErrorCode,
    pub error_details: String,
}

impl CancellationDetails {
    pub fn from_speech_synthesis_result(
        speech_synthesis_result: SpeechSynthesisResult,
    ) -> Result<Self> {
        unsafe {
            let mut reason = 0;
            let mut ret = synth_result_get_reason_canceled(
                speech_synthesis_result.handle.inner(),
                &mut reason,
            );
            convert_err(
                ret,
                "CancellationDetails::from_speech_synthesis_result(reason) error",
            )?;

            let mut error_code = 0;
            ret = synth_result_get_canceled_error_code(
                speech_synthesis_result.handle.inner(),
                &mut error_code,
            );
            convert_err(
                ret,
                "CancellationDetails::from_speech_synthesis_result(error_code) error",
            )?;

            let error_details = speech_synthesis_result
                .properties
                .get_property(PropertyId::CancellationDetailsReasonDetailedText, "")?;

            Ok(CancellationDetails {
                reason: reason.into(),
                error_code: error_code.into(),
                error_details,
            })
        }
    }
}
