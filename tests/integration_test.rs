use cognitive_services_speech_sdk_rs as msspeech;
use log::*;
use rust_embed::Embed;
use std::env;
use std::path::PathBuf;

#[derive(Embed)]
#[folder = "examples/sample_files"]
struct Asset;

#[tokio::test]
async fn speech_to_text() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut file_path = PathBuf::from(&current_dir);
    file_path.push("examples");
    file_path.push("sample_files");
    file_path.push("myVoiceIsMyPassportVerifyMe01.wav");
    let file_path_str = &file_path.into_os_string().into_string().unwrap();
    let audio_config = msspeech::audio::AudioConfig::from_wav_file_input(file_path_str).unwrap();

    // before running this text export the below listed variables. Example:
    // export MSSubscriptionKey=32...
    // export MSServiceRegion=westeurope
    let speech_config = msspeech::speech::SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap_or("westeurope".to_string()),
    )
    .unwrap();
    let mut speech_recognizer =
        msspeech::speech::SpeechRecognizer::from_config(speech_config, audio_config).unwrap();

    speech_recognizer
        .set_session_started_cb(|event| info!("set_session_started_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_session_stopped_cb(|event| info!("set_session_stopped_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_speech_start_detected_cb(|event| info!("set_speech_start_detected_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_speech_end_detected_cb(|event| info!("set_speech_end_detected_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_recognizing_cb(|event| info!("set_recognizing_cb {:?}", event.result.text))
        .unwrap();

    speech_recognizer
        .set_recognized_cb(|event| info!("set_recognized_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_canceled_cb(|event| info!("set_canceled_cb {:?}", event))
        .unwrap();

    let result = speech_recognizer.recognize_once_async().await.unwrap();
    println!("got recognition {:?}", result);
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
    let pull_stream = msspeech::audio::PullAudioOutputStream::create_pull_stream().unwrap();
    let audio_config = msspeech::audio::AudioConfig::from_stream_output(&pull_stream).unwrap();

    // before running this text export the below listed variables. Example:
    // export MSSubscriptionKey=32...
    // export MSServiceRegion=westeurope
    // cargo test
    let speech_config = msspeech::speech::SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let mut speech_synthesizer =
        msspeech::speech::SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();

    speech_synthesizer
        .set_synthesizer_started_cb(|event| info!("synthesizer_started_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_synthesizing_cb(|event| info!("synthesizer_synthesizing_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_completed_cb(|event| info!("synthesizer_completed_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_canceled_cb(|event| info!("synthesizer_canceled_cb {:?}", event))
        .unwrap();

    match speech_synthesizer.speak_text_async("Hello Rust!").await {
        Err(err) => error!("speak_text_async error {:?}", err),
        Ok(speech_audio_bytes) => {
            info!("speech_audio_bytes {:?}", speech_audio_bytes);
        }
    }
}
