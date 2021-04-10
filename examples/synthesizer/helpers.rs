use cognitive_services_speech_sdk_rs::audio::{
    AudioConfig, AudioStreamFormat, PullAudioOutputStream, PushAudioInputStream,
};
use cognitive_services_speech_sdk_rs::speech::{SpeechConfig, SpeechSynthesizer};
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
