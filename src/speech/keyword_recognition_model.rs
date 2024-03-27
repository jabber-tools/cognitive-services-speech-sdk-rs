use crate::error::{convert_err, Result};

use crate::ffi::{
    keyword_recognition_model_create_from_file, keyword_recognition_model_handle_release,
    SmartHandle, SPXKEYWORDHANDLE,
};

use std::ffi::CString;
use std::mem::MaybeUninit;

/// KeywordRecognitionModel represents the keyword recognition model used with start_keyword_recognition_async methods.
#[derive(Debug)]
pub struct KeywordRecognitionModel {
    pub handle: SmartHandle<SPXKEYWORDHANDLE>,
}

impl KeywordRecognitionModel {
    fn from_handle(handle: SPXKEYWORDHANDLE) -> Result<KeywordRecognitionModel> {
        Ok(KeywordRecognitionModel {
            handle: SmartHandle::create(
                "KeywordRecognitionModel",
                handle,
                keyword_recognition_model_handle_release,
            ),
        })
    }

    pub fn from_file(filename: &str) -> Result<KeywordRecognitionModel> {
        unsafe {
            let c_filename = CString::new(filename)?;
            let mut handle = MaybeUninit::uninit();
            let ret = keyword_recognition_model_create_from_file(
                c_filename.as_ptr(),
                handle.as_mut_ptr(),
            );
            convert_err(ret, "KeywordRecognitionModel::from_file error")?;
            KeywordRecognitionModel::from_handle(handle.assume_init())
        }
    }
}
