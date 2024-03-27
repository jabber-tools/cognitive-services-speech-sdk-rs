use crate::audio::PullAudioOutputStream;
use crate::error::{convert_err, Result};
use crate::ffi::{
    dialog_service_connector_activity_received_event_get_activity,
    dialog_service_connector_activity_received_event_get_activity_size,
    dialog_service_connector_activity_received_event_get_audio,
    dialog_service_connector_activity_received_event_has_audio,
    dialog_service_connector_activity_received_event_release, SmartHandle, SPXEVENTHANDLE,
};
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

/// Event passed into callback registered by *DialogServiceConnector::set_activity_received_cb*.
#[derive(Debug)]
pub struct ActivityReceivedEvent {
    pub handle: SmartHandle<SPXEVENTHANDLE>,
    pub activity: String,
}

impl ActivityReceivedEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<ActivityReceivedEvent> {
        unsafe {
            let mut size = MaybeUninit::uninit();
            let mut ret = dialog_service_connector_activity_received_event_get_activity_size(
                handle,
                size.as_mut_ptr(),
            );
            convert_err(ret, "ActivityReceivedEvent::from_handle(get size) error")?;
            let size = size.assume_init();
            // cannot initiate array with dynamic size (i.e. [0u8; size + 1] )
            // -> allocate vector and convert it to slice
            let mut buf_vec = vec![0u8; size + 1];
            let c_buf: *mut c_char = &mut buf_vec[..] as *const _ as *mut c_char;
            ret =
                dialog_service_connector_activity_received_event_get_activity(handle, c_buf, size);
            convert_err(
                ret,
                "ActivityReceivedEvent::from_handle(get activity) error",
            )?;
            let activity = CStr::from_ptr(c_buf).to_str()?.to_owned();
            Ok(ActivityReceivedEvent {
                handle: SmartHandle::create(
                    "ActivityReceivedEvent",
                    handle,
                    dialog_service_connector_activity_received_event_release,
                ),
                activity,
            })
        }
    }

    pub fn has_audio(&self) -> bool {
        unsafe { dialog_service_connector_activity_received_event_has_audio(self.handle.inner()) }
    }

    pub fn get_audio(&self) -> Result<PullAudioOutputStream> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            let ret = dialog_service_connector_activity_received_event_get_audio(
                self.handle.inner(),
                handle.as_mut_ptr(),
            );
            convert_err(ret, "ActivityReceivedEvent.get_audio error")?;
            PullAudioOutputStream::from_handle(handle.assume_init())
        }
    }
}
