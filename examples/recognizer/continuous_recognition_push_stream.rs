use super::helpers;
use log::*;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("running continuous_recognition_push_stream example...");

    let filename = helpers::get_sample_file("chinese_test.wav");

    let (mut speech_recognizer, audio_push_stream) = helpers::speech_recognizer_from_push_stream();
    helpers::set_callbacks(&mut speech_recognizer);

    let handle = tokio::spawn(async move {
        if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
            error!("start_continuous_recognition_async error {:?}", err);
        }
        sleep(Duration::from_millis(10000)).await;
    });

    helpers::push_file_into_stream(&filename, audio_push_stream);
    handle.await.unwrap();

    info!("example finished!");
}
