mod common;
use std::time::Duration;

use cognitive_services_speech_sdk_rs::speech::SpeechSynthesizer;
use common::*;
use log::{error, info};
use tokio::time::sleep;

#[tokio::test]
async fn tts_segfault_test() {
    let tts = Tts::new();
    tts.start("In Rust, we trust.").await;

    sleep(Duration::from_secs(5)).await;

    tts.stop().await;
}

struct Tts {
    synthesizer: SpeechSynthesizer,
}

impl Tts {
    pub fn new() -> Self {
        let (mut speech_synthesizer, _) = speech_synthesizer_pull();

        set_synthesizer_callbacks(&mut speech_synthesizer);

        Self {
            synthesizer: speech_synthesizer,
        }
    }

    async fn start(&self, text: &str) {
        let result = self
            .synthesizer
            .start_speaking_text_async(text)
            .await
            .unwrap();
        info!("Synthesis result: {result:?}");
    }

    async fn stop(&self) {
        if let Err(err) = self.synthesizer.stop_speaking_async().await {
            error!("stop_speaking_async error {err:?}");
        }
    }
}
