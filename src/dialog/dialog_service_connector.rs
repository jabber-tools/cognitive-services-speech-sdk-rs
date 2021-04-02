use crate::audio::AudioConfig;
use crate::common::PropertyCollection;
use crate::dialog::DialogServiceConfig;
use crate::error::{convert_err, Result};
use crate::ffi::{
    dialog_service_connector_create_dialog_service_connector_from_config,
    dialog_service_connector_get_property_bag, dialog_service_connector_handle_release,
    SmartHandle, SPXHANDLE, SPXPROPERTYBAGHANDLE, SPXRECOHANDLE,
};
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct DialogServiceConnector {
    pub properties: PropertyCollection,
    pub handle: SmartHandle<SPXRECOHANDLE>,
}

impl DialogServiceConnector {
    fn from_handle(handle: SPXRECOHANDLE) -> Result<Self> {
        unsafe {
            let mut prop_bag_handle: SPXPROPERTYBAGHANDLE = MaybeUninit::uninit().assume_init();
            let ret = dialog_service_connector_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "DialogServiceConnector::from_handle error")?;
            Ok(DialogServiceConnector {
                properties: PropertyCollection::from_handle(prop_bag_handle),
                handle: SmartHandle::create(
                    "DialogServiceConnector",
                    handle,
                    dialog_service_connector_handle_release,
                ),
            })
        }
    }

    // using static dispatch, see https://joshleeb.com/posts/rust-traits-and-trait-objects/
    pub fn from_config(
        dialog_service_config: impl DialogServiceConfig,
        audio_config: Option<AudioConfig>,
    ) -> Result<Self> {
        unsafe {
            let mut handle: SPXRECOHANDLE = MaybeUninit::uninit().assume_init();
            let speech_config_handle = dialog_service_config.get_handle();
            let ret;
            if let Some(audio_cfg) = audio_config {
                ret = dialog_service_connector_create_dialog_service_connector_from_config(
                    &mut handle,
                    speech_config_handle,
                    audio_cfg.handle.inner(),
                );
            } else {
                let spxhandle_null: SPXHANDLE = 0 as SPXHANDLE;
                ret = dialog_service_connector_create_dialog_service_connector_from_config(
                    &mut handle,
                    speech_config_handle,
                    spxhandle_null,
                );
            }
            convert_err(ret, "DialogServiceConnector::from_config error")?;
            DialogServiceConnector::from_handle(handle)
        }
    }
}
