use crate::error::{convert_err, Result};
use crate::ffi::{
    recognizer_event_handle_release, recognizer_session_event_get_session_id, SmartHandle,
    SPXEVENTHANDLE,
};
use log::*;
use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;

/// Base *SpeechRecognizer* event passed into callbacks *set_session_started_cb* and *set_session_stopped_cb*.
pub struct SessionEvent {
    pub session_id: String,
    pub handle: SmartHandle<SPXEVENTHANDLE>,
}

/// custom Debug implementation for SessionEvent so that
/// handle is not included. If needed, disable this impl
/// and reenable derive Debug above
impl fmt::Debug for SessionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionEvent")
            .field("session_id", &self.session_id)
            .finish()
    }
}

impl SessionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SessionEvent> {
        let mut buffer: [u8; 37] = [0; 37];

        unsafe {
            let c_buf: *mut c_char = &mut buffer as *const _ as *mut c_char;
            trace!("calling recognizer_session_event_get_session_id");
            let ret = recognizer_session_event_get_session_id(handle, c_buf, 37);
            convert_err(ret, "SessionEvent::from_handle error")?;
            trace!("called recognizer_session_event_get_session_id");

            let c_str: &CStr = CStr::from_ptr(c_buf);
            let str_slice: &str = c_str.to_str()?;
            let str_buf: String = str_slice.to_owned();
            trace!("converted cstring to owned string");

            Ok(SessionEvent {
                session_id: str_buf,
                handle: SmartHandle::create(
                    "SessionEvent",
                    handle,
                    recognizer_event_handle_release,
                ),
            })
        }
    }
}
