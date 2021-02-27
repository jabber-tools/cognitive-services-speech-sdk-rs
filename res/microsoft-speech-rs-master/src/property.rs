use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::convert_err;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;

const NULL_C_STR_PTR: *const c_char = 0 as *const c_char;

pub enum PropertyId
{
    SpeechServiceConnectionKey = 1000,
    SpeechServiceConnectionEndpoint = 1001,
    SpeechServiceConnectionRegion = 1002,
    SpeechServiceAuthorizationToken = 1003,
    SpeechServiceAuthorizationType = 1004,
    SpeechServiceConnectionEndpointId = 1005,

    SpeechServiceConnectionProxyHostName = 1100,
    SpeechServiceConnectionProxyPort = 1101,
    SpeechServiceConnectionProxyUserName = 1102,
    SpeechServiceConnectionProxyPassword = 1103,

    SpeechServiceConnectionTranslationToLanguages = 2000,
    SpeechServiceConnectionTranslationVoice = 2001,
    SpeechServiceConnectionTranslationFeatures = 2002,
    SpeechServiceConnectionIntentRegion = 2003,

    SpeechServiceConnectionRecoMode = 3000,
    SpeechServiceConnectionRecoLanguage = 3001,
    SpeechSessionId = 3002,

    SpeechServiceConnectionSynthLanguage = 3100,
    SpeechServiceConnectionSynthVoice = 3101,
    SpeechServiceConnectionSynthOutputFormat = 3102,

    SpeechServiceResponseRequestDetailedResultTrueFalse = 4000,
    SpeechServiceResponseRequestProfanityFilterTrueFalse = 4001,

    SpeechServiceResponseJsonResult = 5000,
    SpeechServiceResponseJsonErrorDetails = 5001,
    SpeechServiceResponseRecognitionLatencyMs = 5002,

    CancellationDetailsReason = 6000,
    CancellationDetailsReasonText = 6001,
    CancellationDetailsReasonDetailedText = 6002,

    LanguageUnderstandingServiceResponseJsonResult = 7000,

    AudioConfigDeviceNameForCapture = 8000,
    AudioConfigNumberOfChannelsForCapture = 8001,
    AudioConfigSampleRateForCapture = 8002,
    AudioConfigBitsPerSampleForCapture = 8003,
    AudioConfigAudioSource = 8004,

    SpeechLogFilename = 9001,
}

#[derive(Debug)]
pub struct PropertyBag {
    handle: SmartHandle<SPXPROPERTYBAGHANDLE>,
}

impl PropertyBag {
    #[inline(always)]
    pub(crate)
    fn create(hcfg: SPXHANDLE,
              f: unsafe extern "C" fn(SPXHANDLE, *mut SPXPROPERTYBAGHANDLE) -> SPXHR) -> Result<PropertyBag, SpxError> {
        let handle = crate::spx_populate(hcfg, f)?;
        Ok(PropertyBag {
            handle: SmartHandle::create("PropertyBag", handle, property_bag_release),
        })
    }

    #[inline]
    pub fn get_handle(&self) -> SPXPROPERTYBAGHANDLE {
        self.handle.get()
    }

    pub fn get(&self, id: PropertyId) -> Result<Option<String>, SpxError> {
        unsafe {
            let ret = property_bag_get_string(self.get_handle(), id as i32, NULL_C_STR_PTR, NULL_C_STR_PTR);
            return if ret == NULL_C_STR_PTR {
                Ok(None)
            } else {
                Ok(Some(CStr::from_ptr(ret).to_string_lossy().into_owned()))
            };
        }
    }

    pub fn set<T: AsRef<str>>(&mut self, id: PropertyId, v: T) -> Result<(), SpxError> {
        let s = CString::new(v.as_ref())?;
        unsafe {
            convert_err(property_bag_set_string(self.get_handle(), id as i32, NULL_C_STR_PTR, s.as_ptr()))?;
        }
        Ok(())
    }
}
