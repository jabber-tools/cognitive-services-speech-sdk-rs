use cognitive_services_speech_sdk_rs::audio::AudioConfig;
use cognitive_services_speech_sdk_rs::audio::{AudioStreamFormat, PushAudioInputStream};
use cognitive_services_speech_sdk_rs::speech::{SpeechConfig, SpeechRecognizer};
use log::*;
use std::env;
use std::io::Read;
use std::time::Duration;
use tokio::time::sleep;

/// convenience function tosetup environment variables
fn set_env_vars() {
    let msskey: String = std::fs::read_to_string("/home/adambe/projects/mskey")
        .unwrap()
        .trim()
        .to_owned();

    env::set_var("MSSubscriptionKey", msskey);
    env::set_var("MSServiceRegion", "westeurope");
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
}

fn set_callbacks(speech_recognizer: &mut SpeechRecognizer) {
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
}

///creates speech recognizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
fn speech_recognizer_from_audio_cfg(audio_config: AudioConfig) -> SpeechRecognizer {
    trace!("calling SpeechConfig::from_subscription");
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    trace!("called SpeechConfig::from_subscription {:?}", speech_config);

    trace!("calling SpeechRecognizer::from_config");
    let speech_recognizer = SpeechRecognizer::from_config(speech_config, audio_config).unwrap();
    trace!(
        "called SpeechRecognizer::from_config {:?}",
        speech_recognizer
    );
    speech_recognizer
}

#[allow(dead_code)]
/// creates speech recognizer from push input stream and MS speech subscription key
fn speech_recognizer_from_push_stream() -> (SpeechRecognizer, PushAudioInputStream) {
    trace!("calling AudioStreamFormat::get_wave_format_pcm");
    let wave_format = AudioStreamFormat::get_wave_format_pcm(16000, None, None).unwrap();
    trace!(
        "called AudioStreamFormat::get_wave_format_pcm {:?}",
        wave_format
    );

    trace!("calling AudioInputStream::create_push_stream_from_format");
    let push_stream = PushAudioInputStream::create_push_stream_from_format(wave_format).unwrap();
    trace!(
        "called AudioInputStream::create_push_stream_from_format {:?}",
        push_stream
    );

    trace!("calling AudioConfig::from_stream_input");
    let audio_config = AudioConfig::from_stream_input(&push_stream).unwrap();
    trace!("called AudioConfig::from_stream_input {:?}", audio_config);

    (speech_recognizer_from_audio_cfg(audio_config), push_stream)
}

/// creates speech recognizer from wav input file and MS speech subscription key
fn speech_recognizer_from_wav_file() -> SpeechRecognizer {
    // let wav_file = "/home/adambe/projects/microsoft-speech-rs-master/examples/hello_rust.wav";
    let wav_file = "/home/adambe/projects/microsoft-speech-rs-master/examples/chinese_test.wav";

    trace!("calling AudioConfig::from_wav_file_input");
    let audio_config = AudioConfig::from_wav_file_input(wav_file).unwrap();
    trace!("called AudioConfig::from_wav_file_input {:?}", audio_config);

    speech_recognizer_from_audio_cfg(audio_config)
}

/// creates speech recognizer from default mic settings and MS speech subscription key
fn speech_recognizer_default_mic() -> SpeechRecognizer {
    trace!("calling AudioConfig::from_default_microphone_input");
    let audio_config = AudioConfig::from_default_microphone_input().unwrap();
    trace!(
        "called AudioConfig::from_default_microphone_input {:?}",
        audio_config
    );

    speech_recognizer_from_audio_cfg(audio_config)
}

#[allow(dead_code)]
/// creates recognizer for recognition from default mice. not really working on WLS
async fn from_microphone() {
    let mut speech_recognizer = speech_recognizer_default_mic();

    set_callbacks(&mut speech_recognizer);

    if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
        error!("start_continuous_recognition_async error {:?}", err);
    }
    sleep(Duration::from_millis(20000)).await;
}

#[allow(dead_code)]
/// sample for recognize_once_async
async fn recognize_once() {
    let mut speech_recognizer = speech_recognizer_from_wav_file();
    let speech_reco_res = speech_recognizer.recognize_once_async().await;
    info!("got recognition {:?}", speech_reco_res);
}

#[allow(dead_code)]
/// sample for start_continuous_recognition_async
async fn continuous_recognition() {
    let mut speech_recognizer = speech_recognizer_from_wav_file();

    set_callbacks(&mut speech_recognizer);

    if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
        error!("start_continuous_recognition_async error {:?}", err);
    }
    sleep(Duration::from_millis(10000)).await;
}

#[allow(dead_code)]
async fn continuous_recognition_push_stream() {
    let (mut speech_recognizer, mut audio_push_stream) = speech_recognizer_from_push_stream();

    set_callbacks(&mut speech_recognizer);

    let handle = tokio::spawn(async move {
        if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
            error!("start_continuous_recognition_async error {:?}", err);
        }
        sleep(Duration::from_millis(10000)).await;
    });

    // let wav_file = "/home/adambe/projects/microsoft-speech-rs-master/examples/hello_rust.wav";
    let wav_file = "/home/adambe/projects/microsoft-speech-rs-master/examples/chinese_test.wav";

    let mut file = std::fs::File::open(wav_file).unwrap();
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

    handle.await.unwrap();
}

#[allow(dead_code)]
async fn continuous_recognition_push_stream_once() {
    let (mut speech_recognizer, mut audio_push_stream) = speech_recognizer_from_push_stream();

    set_callbacks(&mut speech_recognizer);

    // let wav_file = "/home/adambe/projects/microsoft-speech-rs-master/examples/hello_rust.wav";
    let wav_file = "/home/adambe/projects/microsoft-speech-rs-master/examples/chinese_test.wav";

    let mut file = std::fs::File::open(wav_file).unwrap();
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

    let speech_reco_res = speech_recognizer.recognize_once_async().await;
    info!("got recognition {:?}", speech_reco_res);
}

#[tokio::main]
async fn main() {
    set_env_vars();
    env_logger::init();

    info!("running recognition!!!");
    // from_microphone().await;
    // recognize_once().await;
    continuous_recognition().await;
    // continuous_recognition_push_stream().await;
    // continuous_recognition_push_stream_once().await;
    info!("DONE!");
}
