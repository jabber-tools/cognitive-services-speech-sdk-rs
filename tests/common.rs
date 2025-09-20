#![allow(dead_code)]
use cognitive_services_speech_sdk_rs::audio::{
    AudioConfig, AudioStreamFormat, PullAudioInputStream, PullAudioOutputStream,
    PushAudioOutputStream,
};
use cognitive_services_speech_sdk_rs::speech::{SpeechConfig, SpeechRecognizer, SpeechSynthesizer};
use log::*;
use std::env;
use std::path::PathBuf;

pub(crate) fn get_sample_file(file_name: &str) -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut file_path = PathBuf::from(&current_dir);
    file_path.push("examples");
    file_path.push("sample_files");
    file_path.push(file_name);
    file_path.into_os_string().into_string().unwrap()
}

/// creates speech recognizer from pull input stream and MS speech subscription key
/// returns recognizer and also pull stream so that data push can be initiated
pub(crate) fn speech_recognizer_from_pull_stream() -> (SpeechRecognizer, PullAudioInputStream) {
    let wave_format = AudioStreamFormat::get_wave_format_pcm(16000, None, None).unwrap();
    let pull_stream = PullAudioInputStream::from_format(&wave_format).unwrap();
    let audio_config = AudioConfig::from_stream_input(&pull_stream).unwrap();
    (speech_recognizer_from_audio_cfg(audio_config), pull_stream)
}

/// creates speech recognizer from wav input file and MS speech subscription key
pub(crate) fn speech_recognizer_from_wav_file(wav_file: &str) -> SpeechRecognizer {
    let audio_config = AudioConfig::from_wav_file_input(wav_file).unwrap();
    speech_recognizer_from_audio_cfg(audio_config)
}

///creates speech recognizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
pub(crate) fn speech_recognizer_from_audio_cfg(audio_config: AudioConfig) -> SpeechRecognizer {
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();

    SpeechRecognizer::from_config(speech_config, audio_config).unwrap()
}

pub(crate) fn set_recognizer_callbacks(speech_recognizer: &mut SpeechRecognizer) {
    speech_recognizer
        .set_session_started_cb(|event| info!(">set_session_started_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_session_stopped_cb(|event| info!(">set_session_stopped_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_speech_start_detected_cb(|event| info!(">set_speech_start_detected_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_speech_end_detected_cb(|event| info!(">set_speech_end_detected_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_recognizing_cb(|event| info!(">set_recognizing_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_recognized_cb(|event| info!(">set_recognized_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_canceled_cb(|event| info!(">set_canceled_cb {event:?}"))
        .unwrap();
}

///creates speech synthesizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
pub(crate) fn speech_synthesizer_pull() -> (SpeechSynthesizer, PullAudioOutputStream) {
    let pull_stream = PullAudioOutputStream::create_pull_stream().unwrap();
    let audio_config = AudioConfig::from_stream_output(&pull_stream).unwrap();

    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let speech_synthesizer = SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();
    (speech_synthesizer, pull_stream)
}

pub(crate) fn speech_synthesizer_push() -> (SpeechSynthesizer, PushAudioOutputStream) {
    let push_stream = PushAudioOutputStream::create_push_stream().unwrap();
    let audio_config = AudioConfig::from_stream_output(&push_stream).unwrap();

    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let speech_synthesizer = SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();
    (speech_synthesizer, push_stream)
}

pub(crate) fn set_synthesizer_callbacks(speech_synthesizer: &mut SpeechSynthesizer) {
    speech_synthesizer
        .set_synthesizer_started_cb(|event| info!(">synthesizer_started_cb {event:?}"))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_synthesizing_cb(|event| info!(">synthesizer_synthesizing_cb {event:?}"))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_completed_cb(|event| info!(">synthesizer_completed_cb {event:?}"))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_canceled_cb(|event| info!(">synthesizer_canceled_cb {event:?}"))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_word_boundary_cb(|event| {
            info!(">set_synthesizer_word_boundary_cb {event:?}")
        })
        .unwrap();
}
