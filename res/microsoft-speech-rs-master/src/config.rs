use std::ffi::CString;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::convert_err;
use crate::PropertyBag;
use crate::SmartHandle;
use crate::speech_api::*;
use crate::SpxError;
use crate::SPXHANDLE_INVALID;

#[derive(Debug)]
pub struct SpeechConfig {
    handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    props: PropertyBag,
}

impl SpeechConfig {
    pub fn from_subscription<S1, S2>(subscription: S1, region: S2) -> Result<SpeechConfig, SpxError>
        where S1: Into<Vec<u8>>, S2: Into<Vec<u8>> {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;
        Self::create(|handle| {
            unsafe {
                convert_err(speech_config_from_subscription(handle, c_sub.as_ptr(), c_region.as_ptr()))
            }
        })
    }

    pub fn from_endpoint<S1, S2>(endpoint: S1, subscription: S2) -> Result<SpeechConfig, SpxError>
        where S1: Into<Vec<u8>>, S2: Into<Vec<u8>> {
        let c_ep = CString::new(endpoint)?;
        let c_sub = CString::new(subscription)?;
        Self::create(|handle| {
            unsafe {
                convert_err(speech_config_from_endpoint(handle, c_ep.as_ptr(), c_sub.as_ptr()))
            }
        })
    }

    #[inline(always)]
    fn create(f: impl FnOnce(&mut SPXHANDLE) -> Result<(), SpxError>) -> Result<SpeechConfig, SpxError> {
        let mut handle = SPXHANDLE_INVALID;
        f(&mut handle)?;
        let result = SpeechConfig {
            handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
            props: PropertyBag::create(handle, speech_config_get_property_bag)?,
        };
        Ok(result)
    }

    #[inline]
    pub fn get_handle(&self) -> SPXSPEECHCONFIGHANDLE {
        self.handle.get()
    }

    pub fn set_audio_output_format(&mut self, format: SpeechSynthesisOutputFormat) -> Result<(), SpxError> {
        unsafe {
            convert_err(speech_config_set_audio_output_format(self.get_handle(), format as u32))
        }
    }
}

impl Deref for SpeechConfig {
    type Target = PropertyBag;

    fn deref(&self) -> &Self::Target {
        &self.props
    }
}

impl DerefMut for SpeechConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.props
    }
}

pub enum SpeechSynthesisOutputFormat {
    // raw-8khz-8bit-mono-mulaw
    Raw8Khz8BitMonoMULaw = 1,

    // riff-16khz-16kbps-mono-siren
    Riff16Khz16KbpsMonoSiren = 2,

    // audio-16khz-16kbps-mono-siren
    Audio16Khz16KbpsMonoSiren = 3,

    // audio-16khz-32kbitrate-mono-mp3
    Audio16Khz32KBitRateMonoMp3 = 4,

    // audio-16khz-128kbitrate-mono-mp3
    Audio16Khz128KBitRateMonoMp3 = 5,

    // audio-16khz-64kbitrate-mono-mp3
    Audio16Khz64KBitRateMonoMp3 = 6,

    // audio-24khz-48kbitrate-mono-mp3
    Audio24Khz48KBitRateMonoMp3 = 7,

    // audio-24khz-96kbitrate-mono-mp3
    Audio24Khz96KBitRateMonoMp3 = 8,

    // audio-24khz-160kbitrate-mono-mp3
    Audio24Khz160KBitRateMonoMp3 = 9,

    // raw-16khz-16bit-mono-truesilk
    Raw16Khz16BitMonoTrueSilk = 10,

    // riff-16khz-16bit-mono-pcm
    Riff16Khz16BitMonoPcm = 11,

    // riff-8khz-16bit-mono-pcm
    Riff8Khz16BitMonoPcm = 12,

    // riff-24khz-16bit-mono-pcm
    Riff24Khz16BitMonoPcm = 13,

    // riff-8khz-8bit-mono-mulaw
    Riff8Khz8BitMonoMULaw = 14,

    // raw-16khz-16bit-mono-pcm
    Raw16Khz16BitMonoPcm = 15,

    // raw-24khz-16bit-mono-pcm
    Raw24Khz16BitMonoPcm = 16,

    // raw-8khz-16bit-mono-pcm
    Raw8Khz16BitMonoPcm = 17,
}
