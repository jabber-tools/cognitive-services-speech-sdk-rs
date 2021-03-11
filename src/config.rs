use crate::error::{convert_err, Result};
use crate::ffi::{
    speech_config_from_subscription, speech_config_get_property_bag, speech_config_release,
    SmartHandle, SPXHANDLE, SPXHANDLE_EMPTY, SPXSPEECHCONFIGHANDLE,
};
use crate::property::PropertyBag;
use std::ffi::CString;

#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyBag,
}

impl SpeechConfig {
    pub fn from_subscription<S1, S2>(subscription: S1, region: S2) -> Result<SpeechConfig>
    where
        S1: Into<Vec<u8>>,
        S2: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;
        Self::create(|handle| unsafe {
            convert_err(
                speech_config_from_subscription(handle, c_sub.as_ptr(), c_region.as_ptr()),
                "SpeechConfig.from_subscription error",
            )
        })
    }

    #[inline(always)]
    fn create(f: impl FnOnce(&mut SPXHANDLE) -> Result<()>) -> Result<SpeechConfig> {
        let mut handle = SPXHANDLE_EMPTY;
        f(&mut handle)?;
        let result = SpeechConfig {
            handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
            properties: PropertyBag::create(handle, speech_config_get_property_bag)?,
        };
        Ok(result)
    }
}
