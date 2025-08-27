use log::{error, *};
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
