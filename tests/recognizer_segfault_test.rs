use cognitive_services_speech_sdk_rs::speech::SpeechRecognizer;
use log::error;
use std::time::Duration;
use tokio::time::sleep;

mod common;
use common::*;

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
