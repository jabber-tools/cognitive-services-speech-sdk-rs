use cognitive_services_speech_sdk_rs::audio::{AudioConfig, PullAudioOutputStream};
use cognitive_services_speech_sdk_rs::speech::{SpeechConfig, SpeechRecognizer, SpeechSynthesizer};
use log::{error, *};
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn speech_to_text() {
    let file_path_str = &get_sample_file("myVoiceIsMyPassportVerifyMe01.wav");
    let mut speech_recognizer = speech_recognizer_from_wav_file(file_path_str);

    set_recognizer_callbacks(&mut speech_recognizer);

    let result = speech_recognizer.recognize_once_async().await.unwrap();
    info!("got recognition {:?}", result);
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
        Err(err) => error!("speak_text_async error {:?}", err),
        Ok(speech_audio_bytes) => {
            info!("speech_audio_bytes {:?}", speech_audio_bytes);
            assert!(speech_audio_bytes.audio_data.len() > 0);
        }
    }
}

#[tokio::test]
async fn stt_segfault_test() {
    let mut stt = Stt::new();
    stt.start().await;

    sleep(Duration::from_secs(5)).await;

    stt.stop().await;
}

struct Stt {
    recognizer: SpeechRecognizer,
}

#[tokio::test]
async fn tts_segfault_test() {
    let tts = Tts::new();
    tts.start("In Rust, we trust.").await;

    sleep(Duration::from_secs(5)).await;

    tts.stop().await;
}

fn get_sample_file(file_name: &str) -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut file_path = PathBuf::from(&current_dir);
    file_path.push("examples");
    file_path.push("sample_files");
    file_path.push(file_name);
    file_path.into_os_string().into_string().unwrap()
}

impl Stt {
    fn new() -> Self {
        let file_path_str = &get_sample_file("turn_on_the_lamp.wav");
        let mut speech_recognizer = speech_recognizer_from_wav_file(file_path_str);

        set_recognizer_callbacks(&mut speech_recognizer);

        Self {
            recognizer: speech_recognizer,
        }
    }

    async fn start(&mut self) {
        if let Err(err) = self.recognizer.start_continuous_recognition_async().await {
            error!("start_continuous_recognition_async error {err:?}");
        }
    }

    async fn stop(&mut self) {
        if let Err(err) = self.recognizer.stop_continuous_recognition_async().await {
            error!("stop_continuous_recognition_async error {err:?}");
        }
    }
}

struct Tts {
    client: SpeechSynthesizer,
}

impl Tts {
    pub fn new() -> Self {
        let (mut speech_synthesizer, _) = speech_synthesizer_pull();

        set_synthesizer_callbacks(&mut speech_synthesizer);

        Self {
            client: speech_synthesizer,
        }
    }

    async fn start(&self, text: &str) {
        let result = self.client.start_speaking_text_async(text).await.unwrap();
        info!("Synthesis result: {result:?}");
    }

    async fn stop(&self) {
        if let Err(err) = self.client.stop_speaking_async().await {
            error!("stop_speaking_async error {err:?}");
        }
    }
}

/// creates speech recognizer from wav input file and MS speech subscription key
fn speech_recognizer_from_wav_file(wav_file: &str) -> SpeechRecognizer {
    let audio_config = AudioConfig::from_wav_file_input(wav_file).unwrap();
    speech_recognizer_from_audio_cfg(audio_config)
}

///creates speech recognizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
fn speech_recognizer_from_audio_cfg(audio_config: AudioConfig) -> SpeechRecognizer {
    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();

    SpeechRecognizer::from_config(speech_config, audio_config).unwrap()
}

fn set_recognizer_callbacks(speech_recognizer: &mut SpeechRecognizer) {
    speech_recognizer
        .set_session_started_cb(|event| info!(">set_session_started_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_session_stopped_cb(|event| info!(">set_session_stopped_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_speech_start_detected_cb(|event| info!(">set_speech_start_detected_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_speech_end_detected_cb(|event| info!(">set_speech_end_detected_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_recognizing_cb(|event| info!(">set_recognizing_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_recognized_cb(|event| info!(">set_recognized_cb {event:?}"))
        .unwrap();

    speech_recognizer
        .set_canceled_cb(|event| info!(">set_canceled_cb {event:?}"))
        .unwrap();
}

///creates speech synthesizer from provided audio config and implicit speech config
/// created from MS subscription key hardcoded in sample file
pub fn speech_synthesizer_pull() -> (SpeechSynthesizer, PullAudioOutputStream) {
    let pull_stream = PullAudioOutputStream::create_pull_stream().unwrap();
    let audio_config = AudioConfig::from_stream_output(&pull_stream).unwrap();

    let speech_config = SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let speech_synthesizer = SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();
    (speech_synthesizer, pull_stream)
}

pub fn set_synthesizer_callbacks(speech_synthesizer: &mut SpeechSynthesizer) {
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

    speech_synthesizer
        .set_synthesizer_word_boundary_cb(|event| {
            info!(">set_synthesizer_word_boundary_cb {:?}", event)
        })
        .unwrap();
}
