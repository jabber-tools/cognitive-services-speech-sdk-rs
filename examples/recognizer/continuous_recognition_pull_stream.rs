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
    fn read(&mut self, data_buffer: &mut [u8], size: u32) -> u32 {
        debug!("BinaryAudioStreamReader.read called");
        // take at most 'size' bytes and read them into data_buffer
        let mut internal_buffer = Vec::new();
        let file_ref = &mut self.file;
        let bytes_read: usize = file_ref
            .take(size as u64)
            .read_to_end(&mut internal_buffer)
            .unwrap();

        // copy read data into target data buffer
        data_buffer.clone_from_slice(&internal_buffer[..]);

        bytes_read as u32
    }

    fn close(&mut self) {
        // nothing to do
        return;
    }

    #[allow(unused_variables)]
    fn get_property(&mut self, id: u32) -> Result<String> {
        unimplemented!();
    }
}

#[allow(dead_code)]
pub async fn run_example() {
    info!("-----------------------------------------------------");
    info!("running continuous_recognition_pull_stream example...");
    info!("-----------------------------------------------------");

    let filename = helpers::get_sample_file("whats_the_weather_like.wav");

    let (mut speech_recognizer, _audio_pull_stream) = helpers::speech_recognizer_from_pull_stream(
        Box::new(BinaryAudioStreamReader::from_file(&filename)),
    );
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
