use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};

use crate::ffi::{
    source_lang_config_from_language, source_lang_config_from_language_and_endpointId,
    source_lang_config_get_property_bag, source_lang_config_release, SmartHandle,
    SPXPROPERTYBAGHANDLE, SPXSOURCELANGCONFIGHANDLE,
};
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct SourceLanguageConfig {
    pub handle: SmartHandle<SPXSOURCELANGCONFIGHANDLE>,
    pub properties: PropertyCollection,
}

impl SourceLanguageConfig {
    fn from_handle(handle: SPXSOURCELANGCONFIGHANDLE) -> Result<SourceLanguageConfig> {
        unsafe {
            let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();

            let ret = source_lang_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SourceLanguageConfig::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle);
            Ok(SourceLanguageConfig {
                handle: SmartHandle::create(
                    "SourceLanguageConfig",
                    handle,
                    source_lang_config_release,
                ),
                properties: property_bag,
            })
        }
    }

    pub fn from_language(language: &str) -> Result<SourceLanguageConfig> {
        unsafe {
            let c_language = CString::new(language)?;
            let mut handle: SPXSOURCELANGCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = source_lang_config_from_language(&mut handle, c_language.as_ptr());
            convert_err(ret, "SourceLanguageConfig::from_language error")?;
            SourceLanguageConfig::from_handle(handle)
        }
    }

    pub fn from_language_and_endpoint_id(
        language: &str,
        endpoint_id: &str,
    ) -> Result<SourceLanguageConfig> {
        unsafe {
            let c_language = CString::new(language)?;
            let c_endpoint_id = CString::new(endpoint_id)?;
            let mut handle: SPXSOURCELANGCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = source_lang_config_from_language_and_endpointId(
                &mut handle,
                c_language.as_ptr(),
                c_endpoint_id.as_ptr(),
            );
            convert_err(
                ret,
                "SourceLanguageConfig::from_language_and_endpoint_id error",
            )?;
            SourceLanguageConfig::from_handle(handle)
        }
    }
}
