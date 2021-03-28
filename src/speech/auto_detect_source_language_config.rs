use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    add_source_lang_config_to_auto_detect_source_lang_config,
    auto_detect_source_lang_config_get_property_bag, auto_detect_source_lang_config_release,
    create_auto_detect_source_lang_config_from_languages,
    create_auto_detect_source_lang_config_from_source_lang_config, SmartHandle,
    SPXAUTODETECTSOURCELANGCONFIGHANDLE, SPXPROPERTYBAGHANDLE,
};
use crate::speech::SourceLanguageConfig;
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct AutoDetectSourceLanguageConfig {
    pub handle: SmartHandle<SPXAUTODETECTSOURCELANGCONFIGHANDLE>,
    pub properties: PropertyCollection,
}

// TBD: implement from_language_configs
impl AutoDetectSourceLanguageConfig {
    fn from_handle(
        handle: SPXAUTODETECTSOURCELANGCONFIGHANDLE,
    ) -> Result<AutoDetectSourceLanguageConfig> {
        unsafe {
            let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();

            let ret = auto_detect_source_lang_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "AutoDetectSourceLanguageConfig::from_handle error")?;

            let property_bag = PropertyCollection::from_handle(prop_bag_handle);
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

    pub fn from_languages(languages: Vec<String>) -> Result<AutoDetectSourceLanguageConfig> {
        unsafe {
            let languages_str = languages.join(",");
            let c_languages_str = CString::new(languages_str)?;
            let mut handle: SPXAUTODETECTSOURCELANGCONFIGHANDLE =
                MaybeUninit::uninit().assume_init();
            let ret = create_auto_detect_source_lang_config_from_languages(
                &mut handle,
                c_languages_str.as_ptr(),
            );
            convert_err(ret, "AutoDetectSourceLanguageConfig::from_languages error")?;
            Ok(AutoDetectSourceLanguageConfig::from_handle(handle)?)
        }
    }

    pub fn from_language_configs(
        languages: Vec<SourceLanguageConfig>,
    ) -> Result<AutoDetectSourceLanguageConfig> {
        unsafe {
            let mut first = true;
            let mut handle: SPXAUTODETECTSOURCELANGCONFIGHANDLE =
                MaybeUninit::uninit().assume_init();
            for language in &languages {
                let ret;
                if first == true {
                    first = false;
                    ret = create_auto_detect_source_lang_config_from_source_lang_config(
                        &mut handle,
                        language.handle.inner(),
                    );
                } else {
                    ret = add_source_lang_config_to_auto_detect_source_lang_config(
                        handle,
                        language.handle.inner(),
                    );
                }
                convert_err(
                    ret,
                    "AutoDetectSourceLanguageConfig::from_language_configs error",
                )?;
            }

            Ok(AutoDetectSourceLanguageConfig::from_handle(handle)?)
        }
    }
}
