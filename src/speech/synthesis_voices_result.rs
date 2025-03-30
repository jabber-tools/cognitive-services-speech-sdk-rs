use crate::common::{PropertyCollection, PropertyId, ResultReason};
use crate::error::{convert_err, Result};
use crate::ffi::{
    synthesis_voices_result_get_property_bag, synthesis_voices_result_get_reason,
    synthesis_voices_result_get_result_id, synthesis_voices_result_get_voice_info,
    synthesis_voices_result_get_voice_num, synthesizer_result_handle_release, SmartHandle,
    SPXPROPERTYBAGHANDLE, SPXRESULTHANDLE,
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
    /// # Safety
    /// `handle` must be a valid reference to a live synthesis voices result.
    pub unsafe fn from_handle(handle: SPXRESULTHANDLE) -> Result<Self> {
        unsafe {
            let c_buf: *mut c_char = &mut [0u8; 1024] as *const _ as *mut c_char;
            let mut ret = synthesis_voices_result_get_result_id(handle, c_buf, 1024);
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_result_id) error",
            )?;
            let result_id = CStr::from_ptr(c_buf).to_str()?.to_owned();

            let mut reason = 0;
            ret = synthesis_voices_result_get_reason(handle, &mut reason);
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(result_get_reason) error",
            )?;

            let mut prop_bag_handle: MaybeUninit<SPXPROPERTYBAGHANDLE> = MaybeUninit::uninit();
            ret = synthesis_voices_result_get_property_bag(handle, prop_bag_handle.as_mut_ptr());
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_property_bag) error",
            )?;

            let properties = PropertyCollection::from_handle(prop_bag_handle.assume_init());

            let error_details =
                properties.get_property(PropertyId::CancellationDetailsReasonDetailedText, "")?;

            let mut voice_num = 0;
            ret = synthesis_voices_result_get_voice_num(handle, &mut voice_num);
            convert_err(
                ret,
                "SynthesisVoicesResult::from_handle(synthesis_voices_result_get_voice_num) error",
            )?;
            let mut voices = vec![];
            for idx in 0..voice_num {
                let mut voice_info_handle: MaybeUninit<SPXRESULTHANDLE> = MaybeUninit::uninit();
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

            Ok(SynthesisVoicesResult {
                handle: SmartHandle::create(
                    "SynthesisVoicesResult",
                    handle,
                    synthesizer_result_handle_release,
                ),
                voices,
                result_id,
                reason: reason.into(),
                error_details,
                properties,
            })
        }
    }
}
