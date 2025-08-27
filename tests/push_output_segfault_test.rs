use std::{sync::mpsc, time::Duration};

use cognitive_services_speech_sdk_rs::{
    audio::{PushAudioOutputStream, PushAudioOutputStreamCallbacks},
    speech::SpeechSynthesizer,
};
use log::{error, info};
use tokio::time::sleep;

mod common;
use common::*;

struct BinaryAudioStreamWriter {
    final_audio_data: Vec<u8>,
    sender: mpsc::Sender<Vec<u8>>,
}

impl BinaryAudioStreamWriter {
    pub fn new(sender: mpsc::Sender<Vec<u8>>) -> Self {
        BinaryAudioStreamWriter {
            final_audio_data: vec![],
            sender,
        }
    }
}

impl PushAudioOutputStreamCallbacks for BinaryAudioStreamWriter {
    fn write(&mut self, data_buffer: &[u8]) -> u32 {
        info!("BinaryAudioStreamWriter::write called");
        let mut data_buffer_vec = data_buffer.to_vec();
        self.final_audio_data.append(&mut data_buffer_vec);
        data_buffer_vec.len() as u32
    }

    fn close(&mut self) {
        info!("BinaryAudioStreamWriter::close called");
        let audio_bytes = self.final_audio_data.clone();
        self.sender.send(audio_bytes).unwrap();
    }
}

struct Tts {
    synthesizer: SpeechSynthesizer,
    // Keep the audio push stream alive
    _audio_push_stream: PushAudioOutputStream,
}

impl Tts {
    pub fn new(tx: mpsc::Sender<Vec<u8>>) -> Self {
        let (speech_synthesizer, mut audio_push_stream) = speech_synthesizer_push();

        audio_push_stream
            .set_callbacks(Box::new(BinaryAudioStreamWriter::new(tx)))
            .unwrap();

        Tts {
            synthesizer: speech_synthesizer,
            _audio_push_stream: audio_push_stream,
        }
    }

    async fn start(&mut self) {
        if let Err(err) = self
            .synthesizer
            .start_speaking_text_async("There's no place like home.")
            .await
        {
            error!("audio_push_stream.start error {err:?}");
        }
    }

    async fn stop(&mut self) {
        if let Err(err) = self.synthesizer.stop_speaking_async().await {
            error!("stop_speaking_async error {err:?}");
        }
    }
}

#[tokio::test]
async fn push_output_audio_stream_segfault_test() {
    let (tx, rx) = mpsc::channel();

    {
        let mut tts = Tts::new(tx);
        tts.start().await;

        sleep(Duration::from_secs(5)).await;

        tts.stop().await;
    }

    // Now the Tts instance is dropped and audio push stream is closed

    let final_audio_data = rx.recv().unwrap();
    assert!(!final_audio_data.is_empty());
}
