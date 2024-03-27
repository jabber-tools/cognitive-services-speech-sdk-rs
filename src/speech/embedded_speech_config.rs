use crate::common::{OutputFormat, ProfanityOption, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    embedded_speech_config_add_path, embedded_speech_config_create,
    embedded_speech_config_get_num_speech_reco_models,
    embedded_speech_config_get_speech_reco_model, property_bag_free_string,
    speech_recognition_model_get_locales, speech_recognition_model_get_name,
    speech_recognition_model_get_path, speech_recognition_model_get_version,
    speech_recognition_model_handle_release, SmartHandle, SPXSPEECHRECOMODELHANDLE,
};
use crate::speech::SpeechConfig;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::path::Path;

/// Class that defines embedded (offline) configurations for speech recognition and speech synthesis.
/// This is currently a preview of the API. The functionality is not available in public releases.
/// Added in version 1.19.0
#[derive(Debug)]
pub struct EmbeddedSpeechConfig {
    pub config: SpeechConfig,
}

impl EmbeddedSpeechConfig {
    /// Creates an instance of the embedded speech config with a specified offline model path
    /// for speech recognition and/or synthesis.
    ///
    /// * `path`: the folder path to search for offline models.
    ///           This can be a root path under which several models are located in sub-folders,
    ///           or a direct path to a specific model folder.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<EmbeddedSpeechConfig> {
        EmbeddedSpeechConfig::from_paths(vec![path])
    }

    /// Creates an instance of the embedded speech config with specified offline model paths
    /// for speech recognition and/or synthesis.
    /// * `paths`: The folder paths to search for offline models.
    ///           These can be root paths under which several models are located in sub-folders,
    ///           or direct paths to specific model folders.
    pub fn from_paths<P: AsRef<Path>>(paths: Vec<P>) -> Result<EmbeddedSpeechConfig> {
        unsafe {
            let mut handle = MaybeUninit::uninit();
            convert_err(
                embedded_speech_config_create(handle.as_mut_ptr()),
                "EmbeddedSpeechConfig::create error",
            )?;

            let handle = handle.assume_init();
            let config = SpeechConfig::from_handle(handle)?;

            for path in paths {
                let c_path = CString::new(path.as_ref().to_string_lossy().as_bytes())?;
                convert_err(
                    embedded_speech_config_add_path(handle, c_path.as_ptr()),
                    "EmbeddedSpeechConfig::add_path error",
                )?;
            }

            Ok(EmbeddedSpeechConfig { config })
        }
    }

    /// Gets a list of available speech recognition models.
    pub fn get_speech_recognition_models(&self) -> Result<Vec<SpeechRecognitionModel>> {
        unsafe {
            let mut count = 0u32;

            convert_err(
                embedded_speech_config_get_num_speech_reco_models(
                    self.config.handle.inner(),
                    &mut count,
                ),
                "EmbeddedSpeechConfig::get_num_models error",
            )?;

            let mut models = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
            for i in 0..count {
                let mut handle = MaybeUninit::uninit();
                convert_err(
                    embedded_speech_config_get_speech_reco_model(
                        self.config.handle.inner(),
                        i,
                        handle.as_mut_ptr(),
                    ),
                    "EmbeddedSpeechConfig::get_model error",
                )?;
                models.push(SpeechRecognitionModel::from_handle(handle.assume_init())?);
            }
            Ok(models)
        }
    }

    /// Sets the model for speech recognition.
    /// * `model`:  The recognition model
    /// * `model_key`: The model decryption key.
    pub fn set_speech_recognition_model<S: Into<String>>(
        &mut self,
        model: &SpeechRecognitionModel,
        model_key: S,
    ) -> Result<()> {
        self.config.set_property(
            PropertyId::SpeechServiceConnectionRecoModelName,
            model.name.clone(),
        )?;
        self.config.set_property(
            PropertyId::SpeechServiceConnectionRecoModelKey,
            model_key.into(),
        )
    }

    /// Gets the model name for speech recognition.
    pub fn get_speech_recognition_model_name(&self) -> Result<String> {
        self.config
            .get_property(PropertyId::SpeechServiceConnectionRecoModelName)
    }

    /// Sets the speech recognition output format.
    /// * `format`: Speech recognition output format (simple or detailed).
    pub fn set_speech_recognition_output_format(&mut self, format: OutputFormat) -> Result<()> {
        self.config.set_get_output_format(format)
    }

    /// Gets the speech recognition output format.
    pub fn get_speech_recognition_output_format(&self) -> Result<OutputFormat> {
        self.config.get_output_format()
    }

    /// Sets the profanity option. This can be used to remove profane words or mask them.
    /// * `profanity`: Sets the profanity option. This can be used to remove profane words or mask them.
    pub fn set_profanity(&mut self, profanity: ProfanityOption) -> Result<()> {
        self.config.set_profanity_option(profanity)
    }

    pub fn set_speech_synthesis_voice<S, K>(&mut self, voice_name: S, model_key: K) -> Result<()>
    where
        S: Into<String>,
        K: Into<String>,
    {
        self.config.set_property(
            PropertyId::SpeechServiceConnectionSynthOfflineVoice,
            voice_name.into(),
        )?;

        self.config.set_property(
            PropertyId::SpeechServiceConnectionSynthModelKey,
            model_key.into(),
        )
    }

    /// Gets the voice name for embedded speech synthesis.
    pub fn get_speech_synthesis_voice_name(&self) -> Result<String> {
        self.config
            .get_property(PropertyId::SpeechServiceConnectionSynthOfflineVoice)
    }

    /// Sets the speech synthesis output format (e.g. Riff16Khz16BitMonoPcm).
    pub fn set_speech_synthesis_output_format<F: Into<String>>(&mut self, format: F) -> Result<()> {
        self.config
            .set_get_speech_synthesis_output_format(format.into())
    }

    /// Gets the speech synthesis output format.
    pub fn get_speech_synthesis_output_format(&self) -> Result<String> {
        self.config.get_speech_synthesis_output_format()
    }
}

#[derive(Debug)]
pub struct SpeechRecognitionModel {
    pub name: String,
    pub version: String,
    pub path: String,
    pub locales: Vec<String>,
}

impl SpeechRecognitionModel {
    fn from_handle(handle: SPXSPEECHRECOMODELHANDLE) -> Result<SpeechRecognitionModel> {
        let handle = SmartHandle::create(
            "SpeechRecognitionModel",
            handle,
            speech_recognition_model_handle_release,
        );

        unsafe {
            let name = to_owned(
                speech_recognition_model_get_name(handle.inner()),
                "SpeechRecognitionModel::from_handle(name) error",
            )?;
            let version = to_owned(
                speech_recognition_model_get_version(handle.inner()),
                "SpeechRecognitionModel::from_handle(version) error",
            )?;
            let path = to_owned(
                speech_recognition_model_get_path(handle.inner()),
                "SpeechRecognitionModel::from_handle(path) error",
            )?;
            let locales = to_owned(
                speech_recognition_model_get_locales(handle.inner()),
                "SpeechRecognitionModel::from_handle(locales) error",
            )?
            .split('|')
            .map(ToString::to_string)
            .collect();

            Ok(SpeechRecognitionModel {
                name,
                version,
                path,
                locales,
            })
        }
    }
}

fn to_owned(c_text: *const ::std::os::raw::c_char, err_msg: &str) -> Result<String> {
    unsafe {
        let text = CStr::from_ptr(c_text).to_str()?.to_owned();
        convert_err(property_bag_free_string(c_text), err_msg)?;
        Ok(text)
    }
}
