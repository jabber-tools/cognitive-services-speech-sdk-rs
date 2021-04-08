use super::helpers;
use cognitive_services_speech_sdk_rs::audio::PullAudioInputStreamCallbacks;
use cognitive_services_speech_sdk_rs::error::Result;
use log::*;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use tokio::time::sleep;

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
        return;
    }

    #[allow(unused_variables)]
    fn get_property(&mut self, id: u32) -> Result<String> {
        debug!("BinaryAudioStreamReader.get_property called {}", id);
        Ok("".to_owned())
    }
}

#[allow(dead_code)]
pub async fn run_example() {
    info!("-----------------------------------------------------");
    info!("running continuous_recognition_pull_stream example...");
    info!("-----------------------------------------------------");

    let filename = helpers::get_sample_file("whats_the_weather_like.wav");

    let (mut speech_recognizer, mut audio_pull_stream) =
        helpers::speech_recognizer_from_pull_stream();

    let reg_all_callbacks = false;
    audio_pull_stream
        .set_callbacks(
            Box::new(BinaryAudioStreamReader::from_file(&filename)),
            reg_all_callbacks,
        )
        .unwrap();

    helpers::set_callbacks(&mut speech_recognizer);

    let handle = tokio::spawn(async move {
        if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
            error!("start_continuous_recognition_async error {:?}", err);
        }
        sleep(Duration::from_millis(10000)).await;
    });

    handle.await.unwrap();

    info!("example finished!");
}
