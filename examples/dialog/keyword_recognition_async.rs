use super::helpers;
use cognitive_services_speech_sdk_rs::audio::{AudioConfig, PushAudioInputStream};
use cognitive_services_speech_sdk_rs::speech::KeywordRecognitionModel;
use log::*;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("running keyword_recognition_async example...");

    let filename_wav = helpers::get_sample_file("whats_the_weather_like.wav");

    let filename_model = helpers::get_sample_file("kws.table");
    let model = KeywordRecognitionModel::from_file(&filename_model).unwrap();

    let input_push_stream = PushAudioInputStream::create_push_stream().unwrap();
    let audio_config = AudioConfig::from_stream_input(&input_push_stream).unwrap();

    let mut dialog_service_connector =
        helpers::dialog_service_connector_from_audio_cfg(audio_config);
    helpers::set_callbacks(&mut dialog_service_connector);

    helpers::push_file_into_stream(&filename_wav, input_push_stream);
    dialog_service_connector
        .start_keyword_recognition_async(&model)
        .await
        .unwrap();
    sleep(Duration::from_millis(10000)).await;

    info!("example finished!");
}
