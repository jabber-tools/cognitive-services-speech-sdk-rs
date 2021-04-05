use super::helpers;
use log::*;

pub async fn run_example() {
    info!("-------------------------------------------------");
    info!("running recognize_once_async_from_file example...");
    info!("-------------------------------------------------");

    let filename = helpers::get_sample_file("hello_rust.wav");
    let mut speech_recognizer = helpers::speech_recognizer_from_wav_file(&filename);
    let speech_reco_res = speech_recognizer.recognize_once_async().await;
    info!("got recognition {:?}", speech_reco_res);
    info!("example finished!");
}
