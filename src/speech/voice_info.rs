use crate::common::{PropertyCollection, SynthesisVoiceType};
use crate::error::{convert_err, Result};
use crate::ffi::{
    voice_info_get_local_name, voice_info_get_locale, voice_info_get_name,
    voice_info_get_property_bag, voice_info_get_short_name, voice_info_get_style_list,
    voice_info_get_voice_path, voice_info_get_voice_type, voice_info_handle_release, SmartHandle,
    SPXRESULTHANDLE,
};
use std::ffi::CStr;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct VoiceInfo {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub name: String,
    pub locale: String,
    pub short_name: String,
    pub local_name: String,
    pub voice_type: SynthesisVoiceType,
    pub style_list: Vec<String>,
    pub voice_path: String,
    pub properties: PropertyCollection,
}

impl VoiceInfo {
    pub fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self> {
        unsafe {
            let c_voice_name = voice_info_get_name(handle);
            let voice_name = CStr::from_ptr(c_voice_name).to_str()?.to_owned();

            let c_locale = voice_info_get_locale(handle);
            let locale = CStr::from_ptr(c_locale).to_str()?.to_owned();

            let c_short_name = voice_info_get_short_name(handle);
            let short_name = CStr::from_ptr(c_short_name).to_str()?.to_owned();

            let c_local_name = voice_info_get_local_name(handle);
            let local_name = CStr::from_ptr(c_local_name).to_str()?.to_owned();

            let c_style_list = voice_info_get_style_list(handle);
            let style_list_str = CStr::from_ptr(c_style_list).to_str()?.to_owned();

            let style_list: Vec<String> =
                style_list_str.split('|').map(|s| s.to_string()).collect();

            let c_voice_path = voice_info_get_voice_path(handle);
            let voice_path = CStr::from_ptr(c_voice_path).to_str()?.to_owned();

            let mut voice_type = MaybeUninit::uninit();
            let mut ret = voice_info_get_voice_type(handle, voice_type.as_mut_ptr());
            convert_err(
                ret,
                "VoiceInfo::from_handle(voice_info_get_voice_type) error",
            )?;

            let mut prop_bag_handle = MaybeUninit::uninit();
            ret = voice_info_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(
                ret,
                "VoiceInfo::from_handle(voice_info_get_property_bag) error",
            )?;

            #[cfg(target_os = "windows")]
            let voice_type = SynthesisVoiceType::from_i32(voice_type.assume_init());
            #[cfg(not(target_os = "windows"))]
            let voice_type = SynthesisVoiceType::from_u32(voice_type.assume_init());

            Ok(VoiceInfo {
                handle: SmartHandle::create("VoiceInfo", handle, voice_info_handle_release),
                name: voice_name,
                locale,
                short_name,
                local_name,
                voice_type,
                style_list,
                voice_path,
                properties: PropertyCollection::from_handle(prop_bag_handle.assume_init()),
            })
        }
    }
}
