use cognitive_services_speech_sdk_rs::audio::AudioConfig;
#[allow(unused_imports)]
use cognitive_services_speech_sdk_rs::audio::{AudioInputStream, AudioStreamFormat};
use cognitive_services_speech_sdk_rs::speech::{SpeechConfig, SpeechRecognizer};
use log::*;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    env::set_var("MSSubscriptionKey", "123456789"); // just for now
    env::set_var("MSServiceRegion", "westeurope");

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    /*
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
    */

    info!("calling AudioConfig::from_wav_file_input");
    let audio_config = AudioConfig::from_wav_file_input(
        "/home/adambe/projects/microsoft-speech-rs-master/examples/hello_rust.wav",
    )
    .unwrap();
    info!("called AudioConfig::from_wav_file_input {:?}", audio_config);

    info!("calling SpeechConfig::from_subscription");
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    info!("called SpeechConfig::from_subscription {:?}", speech_config);

    info!("calling SpeechRecognizer::from_config");
    let mut speech_recognizer = SpeechRecognizer::from_config(speech_config, audio_config).unwrap();
    info!(
        "called SpeechRecognizer::from_config {:?}",
        speech_recognizer
    );

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
        .set_recognizing_cb(|event| info!(">set_recognizing_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_recognized_cb(|event| info!(">set_recognized_cb {:?}", event))
        .unwrap();
    /**/
    let handle = tokio::spawn(async move {
        if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
            error!("start_continuous_recognition_async error {:?}", err);
        }
        sleep(Duration::from_millis(5000)).await;
    });
    handle.await.unwrap();
    info!("DONE!!!");
}
