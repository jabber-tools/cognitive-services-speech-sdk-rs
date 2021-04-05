use super::helpers;
use log::*;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running continuous_recognition_from_file example...");
    info!("---------------------------------------------------");

    let filename = helpers::get_sample_file("turn_on_the_lamp.wav");

    let mut speech_recognizer = helpers::speech_recognizer_from_wav_file(&filename);

    helpers::set_callbacks(&mut speech_recognizer);

    if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
        error!("start_continuous_recognition_async error {:?}", err);
    }
    sleep(Duration::from_millis(10000)).await;
    speech_recognizer
        .stop_continuous_recognition_async()
        .await
        .unwrap();

    info!("example finished!");
}
