use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_get_property_bag, recognizer_handle_release, SmartHandle, SPXHANDLE,
    SPXPROPERTYBAGHANDLE,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct AutoDetectSourceLanguageConfig {
    pub handle: SmartHandle<SPXHANDLE>,
    pub properties: PropertyCollection,
}

fn from_handle(handle: SPXHANDLE) -> Result<AutoDetectSourceLanguageConfig> {
    unsafe {
        let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
        // TBD: auto_detect_source_lang_config_from_languages
        // create_auto_detect_source_lang_config_from_source_lang_config
        // add_source_lang_config_to_auto_detect_source_lang_config
        let ret = recognizer_get_property_bag(handle, &mut prop_bag_handle);
        convert_err(ret, "AutoDetectSourceLanguageConfig::from_handle error")?;

        let property_bag = PropertyCollection::from_handle(prop_bag_handle);
        // TBD: auto_detect_source_lang_config_release
        Ok(AutoDetectSourceLanguageConfig {
            handle: SmartHandle::create("SpeechRecognizer", handle, recognizer_handle_release),
            properties: property_bag,
        })
    }
}
