use cognitive_services_speech_sdk_rs::audio::{
    AudioConfig, PullAudioOutputStream, PushAudioInputStream,
};
use cognitive_services_speech_sdk_rs::speech::{
    SpeechConfig, SpeechRecognitionResult, SpeechRecognizer, SpeechSynthesizer,
};
use log::*;
use std::env;
use std::io::Cursor;
use std::io::Read;

/// convenience function to setup environment variables
/// subscription key is taken from external file
pub fn set_env_vars(ms_key_file_path: &str) {
    let msskey: String = std::fs::read_to_string(ms_key_file_path)
        .unwrap()
        .trim()
        .to_owned();

    env::set_var("MSSubscriptionKey", msskey);
    env::set_var("MSServiceRegion", "westeurope");
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
}

pub fn set_callbacks(speech_synthesizer: &mut SpeechSynthesizer) {
    speech_synthesizer
        .set_synthesizer_started_cb(|event| info!(">synthesizer_started_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_synthesizing_cb(|event| info!(">synthesizer_synthesizing_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_completed_cb(|event| info!(">synthesizer_completed_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_canceled_cb(|event| info!(">synthesizer_canceled_cb {:?}", event))
        .unwrap();
}

pub fn push_bytes_vec_into_stream(bytes_vec: Vec<u8>, mut audio_push_stream: PushAudioInputStream) {
    let chunk_size = 1000;

    let mut bytes_cursor = Cursor::new(bytes_vec);

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = bytes_cursor
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)
            .unwrap();
        if n == 0 {
            break;
        }
        audio_push_stream.write(chunk).unwrap();
        if n < chunk_size {
            break;
        }
    }

    audio_push_stream.close_stream().unwrap();
}

///creates speech synthesizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
pub fn speech_synthesizer() -> SpeechSynthesizer {
    let pull_stream = PullAudioOutputStream::create_pull_stream().unwrap();
    let audio_config = AudioConfig::from_stream_output(&pull_stream).unwrap();

    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let speech_synthesizer = SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();
    speech_synthesizer
}

///creates speech recognizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
pub fn speech_recognizer_from_audio_cfg(audio_config: AudioConfig) -> SpeechRecognizer {
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let speech_recognizer = SpeechRecognizer::from_config(speech_config, audio_config).unwrap();
    speech_recognizer
}

pub fn speech_recognizer_from_push_stream() -> (SpeechRecognizer, PushAudioInputStream) {
    let push_stream = PushAudioInputStream::create_push_stream().unwrap();
    let audio_config = AudioConfig::from_stream_input(&push_stream).unwrap();
    (speech_recognizer_from_audio_cfg(audio_config), push_stream)
}

pub async fn recognize_synthetis_result(synthetis_result: Vec<u8>) -> SpeechRecognitionResult {
    let (mut speech_recognizer, audio_push_stream) = speech_recognizer_from_push_stream();
    push_bytes_vec_into_stream(synthetis_result, audio_push_stream);
    let speech_reco_res = speech_recognizer.recognize_once_async().await;
    speech_reco_res.unwrap()
}
