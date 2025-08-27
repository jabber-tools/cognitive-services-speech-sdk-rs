use std::{ffi::CString, mem::MaybeUninit};

use crate::{
    error::{convert_err, Result},
    ffi::{
        grammar_phrase_create_from_text, grammar_phrase_handle_release, SmartHandle,
        SPXPHRASEHANDLE,
    },
};

/// Represents base class grammar for customizing speech recognition. \
/// Added in version 1.5.0.
#[derive(Debug)]
pub(crate) struct GrammarPhrase {
    pub handle: SmartHandle<SPXPHRASEHANDLE>,
}

impl GrammarPhrase {
    /// Creates a grammar phrase using the specified phrase text.
    /// # Arguments
    /// * `text` - The text representing a phrase that may be spoken by the user.
    pub(crate) fn from_text(text: impl AsRef<str>) -> Result<GrammarPhrase> {
        unsafe {
            let mut handle: MaybeUninit<SPXPHRASEHANDLE> = MaybeUninit::uninit();
            let c_text = CString::new(text.as_ref())?;
            let ret = grammar_phrase_create_from_text(handle.as_mut_ptr(), c_text.as_ptr());
            convert_err(ret, "GrammarPhrase::grammar_phrase_from_text error")?;
            Ok(GrammarPhrase {
                handle: SmartHandle::create(
                    "GrammarPhrase",
                    handle.assume_init(),
                    grammar_phrase_handle_release,
                ),
            })
        }
    }
}
