use cognitive_services_speech_sdk_rs::audio::{
    AudioConfig, AudioStreamFormat, PullAudioInputStream, PushAudioInputStream,
};
use cognitive_services_speech_sdk_rs::speech::{SpeechConfig, SpeechRecognizer};
use log::*;
use std::env;
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

pub fn set_callbacks(speech_recognizer: &mut SpeechRecognizer) {
    speech_recognizer
        .set_session_started_cb(|event| info!(">set_session_started_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_session_stopped_cb(|event| info!(">set_session_stopped_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_speech_start_detected_cb(|event| info!(">set_speech_start_detected_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_speech_end_detected_cb(|event| info!(">set_speech_end_detected_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_recognizing_cb(|event| info!(">set_recognizing_cb {:?}", event.result.text))
        .unwrap();

    speech_recognizer
        .set_recognized_cb(|event| info!(">set_recognized_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_canceled_cb(|event| info!(">set_canceled_cb {:?}", event))
        .unwrap();
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

/// creates speech recognizer from push input stream and MS speech subscription key
/// returns recognizer and also push stream so that data push can be initiated
pub fn speech_recognizer_from_push_stream() -> (SpeechRecognizer, PushAudioInputStream) {
    let wave_format = AudioStreamFormat::get_wave_format_pcm(16000, None, None).unwrap();
    let push_stream = PushAudioInputStream::create_push_stream_from_format(wave_format).unwrap();
    let audio_config = AudioConfig::from_stream_input(&push_stream).unwrap();
    (speech_recognizer_from_audio_cfg(audio_config), push_stream)
}

/// creates speech recognizer from pull input stream and MS speech subscription key
/// returns recognizer and also pull stream so that data push can be initiated
pub fn speech_recognizer_from_pull_stream() -> (SpeechRecognizer, PullAudioInputStream) {
    let wave_format = AudioStreamFormat::get_wave_format_pcm(16000, None, None).unwrap();
    let pull_stream = PullAudioInputStream::from_format(&wave_format).unwrap();
    let audio_config = AudioConfig::from_stream_input(&pull_stream).unwrap();
    (speech_recognizer_from_audio_cfg(audio_config), pull_stream)
}

/// creates speech recognizer from wav input file and MS speech subscription key
pub fn speech_recognizer_from_wav_file(wav_file: &str) -> SpeechRecognizer {
    let audio_config = AudioConfig::from_wav_file_input(wav_file).unwrap();
    speech_recognizer_from_audio_cfg(audio_config)
}

/// creates speech recognizer from default mic settings and MS speech subscription key
pub fn speech_recognizer_default_mic() -> SpeechRecognizer {
    let audio_config = AudioConfig::from_default_microphone_input().unwrap();
    speech_recognizer_from_audio_cfg(audio_config)
}

pub fn push_file_into_stream(filename: &str, mut audio_push_stream: PushAudioInputStream) {
    let mut file = std::fs::File::open(filename).unwrap();
    let chunk_size = 1000;

    loop {
        // info!("pushing");
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = file
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

/// retrieves full path of file with filename
/// from examples/sample_files folder
pub fn get_sample_file(filename: &str) -> String {
    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("examples");
    dir.push("sample_files");
    dir.push(filename);
    dir.into_os_string().into_string().unwrap()
}
