use cognitive_services_speech_sdk_rs::{
    audio::AudioConfig,
    speech::{PhraseListGrammar, SpeechConfig, SpeechRecognizer},
};
use log::{error, *};
use std::{env, path::PathBuf};
mod common;
use common::*;

#[tokio::test]
async fn speech_to_text() {
    let file_path_str = &get_sample_file("myVoiceIsMyPassportVerifyMe01.wav");
    let mut speech_recognizer = speech_recognizer_from_wav_file(file_path_str);

    set_recognizer_callbacks(&mut speech_recognizer);

    let result = speech_recognizer.recognize_once_async().await.unwrap();
    info!("got recognition {result:?}");
    assert!(
        result
            .text
            .to_lowercase()
            .contains("y voice is my passport. verify me")
            || result
                .text
                .to_lowercase()
                .contains("y voice is my passport verify me")
    );
}

#[tokio::test]
async fn text_to_speech() {
    let (mut speech_synthesizer, _) = speech_synthesizer_pull();

    set_synthesizer_callbacks(&mut speech_synthesizer);

    match speech_synthesizer.speak_text_async("Hello Rust!").await {
        Err(err) => error!("speak_text_async error {err:?}"),
        Ok(speech_audio_bytes) => {
            info!("speech_audio_bytes {speech_audio_bytes:?}");
            assert!(!speech_audio_bytes.audio_data.is_empty());
        }
    }
}

#[tokio::test]
async fn phrase_list_test() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut file_path = PathBuf::from(&current_dir);
    file_path.push("examples");
    file_path.push("sample_files");
    file_path.push("peloozoid.wav");
    let file_path_str = &file_path.into_os_string().into_string().unwrap();
    let audio_config = AudioConfig::from_wav_file_input(file_path_str).unwrap();

    // before running this text export the below listed variables. Example:
    // export MSSubscriptionKey=32...
    // export MSServiceRegion=westeurope
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap_or("westeurope".to_string()),
    )
    .unwrap();

    let mut speech_recognizer = SpeechRecognizer::from_config(speech_config, audio_config).unwrap();

    let grammar = PhraseListGrammar::from_recognizer(&speech_recognizer).unwrap();
    grammar.add_phrase("peloozoid").unwrap();

    let result = speech_recognizer.recognize_once_async().await.unwrap();
    // "That shape is a Peloozoid."
    println!(
        "[phrase_list_test] got recognition result: {:?}",
        result.text
    );
    assert!(result.text.to_lowercase().contains("peloozoid"));
}
