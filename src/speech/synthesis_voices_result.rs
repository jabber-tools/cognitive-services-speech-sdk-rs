use crate::common::{PropertyCollection, PropertyId, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesis_voices_result_get_property_bag, synthesis_voices_result_get_reason,
    synthesis_voices_result_get_result_id, synthesis_voices_result_get_voice_info,
    synthesis_voices_result_get_voice_num, synthesizer_result_handle_release, SmartHandle,
    SPXRESULTHANDLE,
};
use crate::speech::VoiceInfo;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct SynthesisVoicesResult {
    pub handle: SmartHandle<SPXRESULTHANDLE>,
    pub voices: Vec<VoiceInfo>,
    pub result_id: String,
    pub reason: ResultReason,
    pub error_details: String,
    pub properties: PropertyCollection,
}

impl SynthesisVoicesResult {
    pub fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self> {
        unsafe {
            let c_buf: *mut c_char = &mut [0u8; 1024] as *const _ as *mut c_char;
            let mut ret = synthesis_voices_result_get_result_id(handle, c_buf, 1024);
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf).to_str()?.to_owned();

            let mut reason = MaybeUninit::uninit();
            ret = synthesis_voices_result_get_reason(handle, reason.as_mut_ptr());
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(result_get_reason) error",
            )?;

            let mut prop_bag_handle = MaybeUninit::uninit();
            ret = synthesis_voices_result_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_property_bag) error",
            )?;

            let properties = PropertyCollection::from_handle(prop_bag_handle.assume_init());

            let error_details =
                properties.get_property(PropertyId::CancellationDetailsReasonDetailedText, "")?;

            let mut voice_num = MaybeUninit::uninit();
            ret = synthesis_voices_result_get_voice_num(handle, voice_num.as_mut_ptr());
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_voice_num) error",
            )?;
            let mut voice_num = voice_num.assume_init();
            voice_num -= 1;
            let mut voices = vec![];
            for idx in 0..voice_num {
                let mut voice_info_handle = MaybeUninit::uninit();
                ret = synthesis_voices_result_get_voice_info(
                    handle,
                    idx,
                    voice_info_handle.as_mut_ptr(),
                );
                convert_err(
                    ret,
                    "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_voice_info) error",
                )?;
                let voice_info = VoiceInfo::from_handle(voice_info_handle.assume_init())?;
                voices.push(voice_info);
            }

            #[cfg(target_os = "windows")]
            let reason = ResultReason::from_i32(reason.assume_init());
            #[cfg(not(target_os = "windows"))]
            let reason = ResultReason::from_u32(reason.assume_init());

            Ok(SynthesisVoicesResult {
                handle: SmartHandle::create(
                    "SynthesisVoicesResult",
                    handle,
                    synthesizer_result_handle_release,
                ),
                voices,
                result_id,
                reason,
                error_details,
                properties,
            })
        }
    }
}
