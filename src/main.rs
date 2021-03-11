use cognitive_services_speech_sdk_rs::audio::{AudioConfig, AudioInputStream, AudioStreamFormat};
use cognitive_services_speech_sdk_rs::config::SpeechConfig;
use cognitive_services_speech_sdk_rs::recognizer::SpeechRecognizer;
use log::*;
use std::env;

fn main() {
    env::set_var("MSSubscriptionKey", "123456789"); // just for now
    env::set_var("MSServiceRegion", "westeurope");

    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("calling AudioStreamFormat::get_wave_format_pcm");
    let wave_format = AudioStreamFormat::get_wave_format_pcm(16000, None, None).unwrap();
    info!(
        "called AudioStreamFormat::get_wave_format_pcm {:?}",
        wave_format
    );

    info!("calling AudioInputStream::create_push_stream_from_format");
    let push_stream = AudioInputStream::create_push_stream_from_format(wave_format).unwrap();
    info!(
        "called AudioInputStream::create_push_stream_from_format {:?}",
        push_stream
    );

    info!("calling AudioConfig::from_stream_input");
    let audio_config = AudioConfig::from_stream_input(push_stream).unwrap();
    info!("called AudioConfig::from_stream_input {:?}", audio_config);

    info!("calling SpeechConfig::from_subscription");
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    info!("called SpeechConfig::from_subscription {:?}", speech_config);

    info!("calling SpeechRecognizer::from_config");
    let speech_recognizer = SpeechRecognizer::from_config(speech_config, audio_config).unwrap();
    info!(
        "called SpeechRecognizer::from_config {:?}",
        speech_recognizer
    );
}
