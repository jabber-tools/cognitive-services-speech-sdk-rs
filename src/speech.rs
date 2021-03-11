use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_release, speech_config_from_subscription, speech_config_get_property_bag,
    speech_config_release, SmartHandle, SPXHANDLE, SPXHANDLE_EMPTY, SPXSPEECHCONFIGHANDLE,
};
use std::ffi::CString;

#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyCollection,
}

impl SpeechConfig {
    fn from_handle(handle: SPXHANDLE) -> Result<SpeechConfig> {
        let mut prop_bag_handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = speech_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechConfig::from_handle error")?;
        }
        let mut property_bag = PropertyCollection {
            handle: SmartHandle::create(
                "PropertyCollection",
                prop_bag_handle,
                property_bag_release,
            ),
        };

        property_bag.set_property_by_string("SPEECHSDK-SPEECH-CONFIG-SYSTEM-LANGUAGE", "Rust")?;

        let result = SpeechConfig {
            handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
            properties: property_bag,
        };
        Ok(result)
    }

    pub fn from_subscription<S>(subscription: S, region: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret =
                speech_config_from_subscription(&mut handle, c_sub.as_ptr(), c_region.as_ptr());
            convert_err(ret, "SpeechConfig::from_subscription error")?
        }
        SpeechConfig::from_handle(handle)
    }
}
