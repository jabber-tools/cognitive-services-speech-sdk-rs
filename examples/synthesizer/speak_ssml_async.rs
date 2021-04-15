use super::helpers;
use cognitive_services_speech_sdk_rs::audio::PushAudioOutputStreamCallbacks;
use log::*;
use std::sync::mpsc;
use std::time::Duration;
use tokio::time::sleep;

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

// uses audio output PUSH stream
#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running speak_ssml_async example...");
    info!("---------------------------------------------------");

    let (mut speech_synthesizer, mut audio_push_stream) = helpers::speech_synthesizer_push();

    let (tx, rx) = mpsc::channel();

    audio_push_stream
        .set_callbacks(Box::new(BinaryAudioStreamWriter::new(tx)))
        .unwrap();

    helpers::set_callbacks(&mut speech_synthesizer);

    let handle = tokio::spawn(async move {
        if let Err(err) = speech_synthesizer
            .speak_ssml_async("<speak xmlns='http://www.w3.org/2001/10/synthesis' xmlns:mstts='http://www.w3.org/2001/mstts' xmlns:emo='http://www.w3.org/2009/10/emotionml' version='1.0' xml:lang='en-US'><voice name='en-GB-George'>This is sample SSML text to transcribe</voice></speak>")
            .await
        {
            error!("speak_ssml_async error {:?}", err);
        }
        sleep(Duration::from_millis(10000)).await;
    });

    handle.await.unwrap();
    let final_audio_data = rx.recv().unwrap();
    // info!("final_audio_data {:?}", final_audio_data);
    let recognition_result = helpers::recognize_synthetis_result(final_audio_data).await;
    info!("recognition_result {:?}", recognition_result);

    info!("example finished!");
}
