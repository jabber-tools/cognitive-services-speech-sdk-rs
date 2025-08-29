use std::{fs::File, io::Read, time::Duration};

use cognitive_services_speech_sdk_rs::{
    audio::{PullAudioInputStream, PullAudioInputStreamCallbacks},
    error::Result,
    speech::SpeechRecognizer,
};
use log::{debug, error, trace};
use tokio::time::sleep;

mod common;
use common::*;

/// simple binary data reader (no error handling at all!)
/// acting as PullAudioInputStream for speech recognizer
/// reading is not optimized at all. The only purpouse is to
/// demonstrate usage of PullAudioInputStream
struct BinaryAudioStreamReader {
    file: File,
}

impl BinaryAudioStreamReader {
    pub fn from_file(filename: &str) -> Self {
        BinaryAudioStreamReader {
            file: File::open(filename).unwrap(),
        }
    }
}

impl PullAudioInputStreamCallbacks for BinaryAudioStreamReader {
    fn read(&mut self, data_buffer: &mut [u8]) -> u32 {
        debug!("BinaryAudioStreamReader.read called {}", data_buffer.len());

        let size = data_buffer.len();

        // take at most 'size' bytes and read them into data_buffer
        let mut internal_buffer = Vec::with_capacity(size);

        let file_ref = &mut self.file;
        let bytes_read: usize = file_ref
            .take(size as u64)
            .read_to_end(&mut internal_buffer)
            .unwrap();

        // if we read less then data_buffer length
        // we need to extend internal_buffer to same length
        // as clone_from_slice requires both slices to have
        // same length
        let diff = size - bytes_read;
        if bytes_read < size {
            for _ in 0..diff {
                internal_buffer.push(0);
            }
        }

        trace!(
            "BinaryAudioStreamReader.internal_buffer {} {}",
            internal_buffer.len(),
            bytes_read
        );
        // copy read data into target data buffer
        data_buffer.clone_from_slice(&internal_buffer[..]);

        bytes_read as u32
    }

    fn close(&mut self) {
        // nothing to do
        debug!("BinaryAudioStreamReader.close called");
    }

    #[allow(unused_variables)]
    fn get_property(&mut self, id: i32) -> Result<String> {
        debug!("BinaryAudioStreamReader.get_property called {id}");
        Ok("".to_owned())
    }
}

struct Stt {
    recognizer: SpeechRecognizer,
    // keep the audio stream alive
    _audio_stream: PullAudioInputStream,
}

impl Stt {
    fn new() -> Self {
        let filename = get_sample_file("turn_on_the_lamp.wav");
        let (speech_recognizer, mut audio_pull_stream) = speech_recognizer_from_pull_stream();

        let reg_all_stream_callbacks = true;
        audio_pull_stream
            .set_callbacks(
                Box::new(BinaryAudioStreamReader::from_file(&filename)),
                reg_all_stream_callbacks,
            )
            .unwrap();

        Self {
            recognizer: speech_recognizer,
            _audio_stream: audio_pull_stream,
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

#[tokio::test]
async fn pull_input_audio_stream_segfault_test() {
    let mut stt = Stt::new();
    stt.start().await;

    sleep(Duration::from_secs(5)).await;

    stt.stop().await;
}
