use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    add_source_lang_config_to_auto_detect_source_lang_config,
    auto_detect_source_lang_config_get_property_bag, auto_detect_source_lang_config_release,
    create_auto_detect_source_lang_config_from_languages,
    create_auto_detect_source_lang_config_from_open_range,
    create_auto_detect_source_lang_config_from_source_lang_config, SmartHandle,
    SPXAUTODETECTSOURCELANGCONFIGHANDLE,
};
use crate::speech::SourceLanguageConfig;
use std::ffi::CString;
use std::mem::MaybeUninit;

/// AutoDetectSourceLanguageConfig defines auto detection source configuration
#[derive(Debug)]
pub struct AutoDetectSourceLanguageConfig {
    pub handle: SmartHandle<SPXAUTODETECTSOURCELANGCONFIGHANDLE>,
    pub properties: PropertyCollection,
}

impl AutoDetectSourceLanguageConfig {
    fn from_handle(handle: SPXAUTODETECTSOURCELANGCONFIGHANDLE) -> Result<Self> {
        unsafe {
            let mut prop_bag_handle = MaybeUninit::uninit();

            let ret = auto_detect_source_lang_config_get_property_bag(
                handle,
                prop_bag_handle.as_mut_ptr(),
            );
            convert_err(ret, "AutoDetectSourceLanguageConfig::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle.assume_init());
            Ok(AutoDetectSourceLanguageConfig {
                handle: SmartHandle::create(
                    "AutoDetectSourceLanguageConfig",
                    handle,
                    auto_detect_source_lang_config_release,
                ),
                properties: property_bag,
            })
        }
    }

    /// Creates an instance of the AutoDetectSourceLanguageConfig with source languages.
    pub fn from_languages(languages: Vec<String>) -> Result<Self> {
        unsafe {
            let languages_str = languages.join(",");
            let c_languages_str = CString::new(languages_str)?;
            let mut handle = MaybeUninit::uninit();
            let ret = create_auto_detect_source_lang_config_from_languages(
                handle.as_mut_ptr(),
                c_languages_str.as_ptr(),
            );
            convert_err(ret, "AutoDetectSourceLanguageConfig::from_languages error")?;
            AutoDetectSourceLanguageConfig::from_handle(handle.assume_init())
        }
    }

    /// Creates an instance of the AutoDetectSourceLanguageConfig with a list of source language config
    pub fn from_language_configs(languages: Vec<SourceLanguageConfig>) -> Result<Self> {
        unsafe {
            let mut first = true;
            let mut handle = MaybeUninit::uninit();
            for language in &languages {
                let ret;
                if first {
                    first = false;
                    ret = create_auto_detect_source_lang_config_from_source_lang_config(
                        handle.as_mut_ptr(),
                        language.handle.inner(),
                    );
                } else {
                    ret = add_source_lang_config_to_auto_detect_source_lang_config(
                        handle.assume_init(),
                        language.handle.inner(),
                    );
                }
                convert_err(
                    ret,
                    "AutoDetectSourceLanguageConfig::from_language_configs error",
                )?;
            }

            AutoDetectSourceLanguageConfig::from_handle(handle.assume_init())
        }
    }

    /// Creates an instance of the AutoDetectSourceLanguageConfig with open range as source languages.
    pub fn from_open_range() -> Result<Self> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = create_auto_detect_source_lang_config_from_open_range(handle.as_mut_ptr());
            convert_err(ret, "AutoDetectSourceLanguageConfig::from_open_range error")?;
            AutoDetectSourceLanguageConfig::from_handle(handle.assume_init())
        }
    }
}
