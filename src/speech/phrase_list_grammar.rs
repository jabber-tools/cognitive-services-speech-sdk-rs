use std::ffi::CString;
use std::mem::MaybeUninit;

use crate::error::{convert_err, Result};
use crate::ffi::{
    grammar_handle_release, grammar_phrase_create_from_text, grammar_phrase_handle_release,
    phrase_list_grammar_add_phrase, phrase_list_grammar_clear,
    phrase_list_grammar_from_recognizer_by_name, SmartHandle, SPXGRAMMARHANDLE, SPXPHRASEHANDLE,
};
use crate::speech::SpeechRecognizer;

/// Represents a phrase list grammar for dynamic grammar scenarios. \
/// Added in version 1.5.0.
#[derive(Debug)]
pub struct PhraseListGrammar {
    handle: SmartHandle<SPXGRAMMARHANDLE>,
}

impl PhraseListGrammar {
    /// Creates a phrase list grammar for the specified recognizer.
    pub fn from_recognizer(recognizer: &SpeechRecognizer) -> Result<PhraseListGrammar> {
        unsafe {
            let c_name = CString::new("")?;
            let mut handle: MaybeUninit<SPXGRAMMARHANDLE> = MaybeUninit::uninit();
            let ret = phrase_list_grammar_from_recognizer_by_name(
                handle.as_mut_ptr(),
                recognizer.handle.inner(),
                c_name.as_ptr(),
            );
            convert_err(ret, "PhraseListGrammar::from_recognizer error")?;
            Ok(PhraseListGrammar {
                handle: SmartHandle::create(
                    "PhraseListGrammar",
                    handle.assume_init(),
                    grammar_handle_release,
                ),
            })
        }
    }

    /// AddPhrase adds a simple phrase that may be spoken by the user.
    pub fn add_phrase(&self, text: impl AsRef<str>) -> Result<()> {
        let grammar_phrase = GrammarPhrase::from_text(text)?;
        let ret: usize = unsafe {
            phrase_list_grammar_add_phrase(self.handle.inner(), grammar_phrase.handle.inner())
        };
        convert_err(ret, "PhraseListGrammar::add_phrase error")?;
        Ok(())
    }

    /// Clears all phrases from the phrase list grammar.
    pub fn clear(&self) -> Result<()> {
        let ret = unsafe { phrase_list_grammar_clear(self.handle.inner()) };
        convert_err(ret, "PhraseListGrammar::clear error")?;
        Ok(())
    }
}

/// Represents base class grammar for customizing speech recognition. \
/// Added in version 1.5.0.
#[derive(Debug)]
struct GrammarPhrase {
    handle: SmartHandle<SPXPHRASEHANDLE>,
}

impl GrammarPhrase {
    /// Creates a grammar phrase using the specified phrase text.
    // # Arguments
    /// * `text` - The text representing a phrase that may be spoken by the user.
    fn from_text(text: impl AsRef<str>) -> Result<GrammarPhrase> {
        unsafe {
            let mut handle: MaybeUninit<SPXPHRASEHANDLE> = MaybeUninit::uninit();
            let c_text = CString::new(text.as_ref())?;
            let ret = grammar_phrase_create_from_text(handle.as_mut_ptr(), c_text.as_ptr());
            convert_err(ret, "GrammarPhrase::grammar_pharse_from_text error")?;
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
