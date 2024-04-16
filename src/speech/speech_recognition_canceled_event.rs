use crate::common::{CancellationErrorCode, CancellationReason, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    result_get_canceled_error_code, result_get_reason_canceled, Result_CancellationErrorCode,
    Result_CancellationReason, SPXEVENTHANDLE,
};
use crate::speech::SpeechRecognitionEvent;
use log::*;
use std::mem::MaybeUninit;

/// Recognition event extending *SpeechRecognitionEvent* passed into callback *set_canceled_cb*.
#[derive(Debug)]
pub struct SpeechRecognitionCanceledEvent {
    pub base: SpeechRecognitionEvent,
    pub reason: CancellationReason,
    pub error_code: CancellationErrorCode,
    pub error_details: String,
}

impl SpeechRecognitionCanceledEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SpeechRecognitionCanceledEvent> {
        let base = SpeechRecognitionEvent::from_handle(handle)?;
        unsafe {
            let mut reason: Result_CancellationReason = MaybeUninit::uninit().assume_init();
            let ret = result_get_reason_canceled(base.result.handle.inner(), &mut reason);
            convert_err(
                ret,
                "SpeechRecognitionCanceledEvent::from_handle(result_get_reason_canceled) error",
            )?;

            let mut error_code: Result_CancellationErrorCode = MaybeUninit::uninit().assume_init();
            let ret = result_get_canceled_error_code(base.result.handle.inner(), &mut error_code);
            convert_err(
                ret,
                "SpeechRecognitionCanceledEvent::from_handle(result_get_canceled_error_code) error",
            )?;

            let error_details;
            let error_details_res = base.result.properties.get_property(
                PropertyId::SpeechServiceResponseJsonErrorDetails,
                "".to_string(),
            );
            if let Err(err) = error_details_res {
                warn!(
                    "Error when getting SpeechServiceResponseJsonErrorDetails {:?}",
                    err
                );
                error_details = "".to_owned();
            } else {
                error_details = error_details_res.unwrap();
            }

            #[cfg(target_os = "windows")]
            {
                Ok(SpeechRecognitionCanceledEvent {
                    base,
                    reason: CancellationReason::from_i32(reason),
                    error_code: CancellationErrorCode::from_i32(error_code),
                    error_details,
                })
            }
            #[cfg(not(target_os = "windows"))]
            {
                Ok(SpeechRecognitionCanceledEvent {
                    base,
                    reason: CancellationReason::from_u32(reason),
                    error_code: CancellationErrorCode::from_u32(error_code),
                    error_details,
                })
            }
        }
    }
}
