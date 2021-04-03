use super::helpers;
use log::*;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("running from_microphone example...");

    let mut speech_recognizer = helpers::speech_recognizer_default_mic();
    helpers::set_callbacks(&mut speech_recognizer);

    if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
        error!("start_continuous_recognition_async error {:?}", err);
    }
    sleep(Duration::from_millis(20000)).await;

    info!("example finished!");
}
