use super::helpers;
use cognitive_services_speech_sdk_rs::audio::AudioConfig;
use cognitive_services_speech_sdk_rs::speech::{EmbeddedSpeechConfig, SpeechRecognizer};
use log::*;
use std::env;

#[allow(dead_code)]
pub async fn run_example() {
    info!("-----------------------------------------------");
    info!("running embedded recognize_from_file example...");
    info!("-----------------------------------------------");

    let filename = helpers::get_sample_file("hello_rust.wav");
    let audio_config = AudioConfig::from_wav_file_input(&filename).unwrap();

    let mut speech_config =
        EmbeddedSpeechConfig::from_path(env::var("ModelPath").unwrap()).unwrap();

    let models = speech_config.get_speech_recognition_models().unwrap();
    let model = models.first().unwrap();
    info!("Using first model: {:?}", model);

    speech_config
        .set_speech_recognition_model(model, env::var("ModelKey").unwrap())
        .unwrap();

    let mut speech_recognizer =
        SpeechRecognizer::from_embedded_config(speech_config, audio_config).unwrap();

    let result = speech_recognizer.recognize_once_async().await;
    info!("got recognition {:?}", result);
    info!("example finished!");
}
