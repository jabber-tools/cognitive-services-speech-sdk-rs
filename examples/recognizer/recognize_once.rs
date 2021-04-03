use super::helpers;
use log::*;

pub async fn run_example() {
    info!("running recognize_once example...");
    let mut speech_recognizer = helpers::speech_recognizer_from_wav_file(helpers::SAMPLE_FILE_1);
    let speech_reco_res = speech_recognizer.recognize_once_async().await;
    info!("got recognition {:?}", speech_reco_res);
    info!("example finished!");
}
