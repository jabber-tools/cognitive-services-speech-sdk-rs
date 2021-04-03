use super::helpers;
use cognitive_services_speech_sdk_rs::audio::{AudioConfig, PushAudioInputStream};
use log::*;

#[allow(dead_code)]
pub async fn run_example() {
    info!("running from_push_audio_stream example...");

    let input_push_stream = PushAudioInputStream::create_push_stream().unwrap();
    let audio_config = AudioConfig::from_stream_input(&input_push_stream).unwrap();

    let mut dialog_service_connector =
        helpers::dialog_service_connector_from_audio_cfg(audio_config);
    helpers::set_callbacks(&mut dialog_service_connector);

    helpers::push_file_into_stream(helpers::SAMPLE_FILE_1, input_push_stream);
    let speech_reco_res = dialog_service_connector.listen_once_async().await;

    info!("speech_reco_res {:#?}", speech_reco_res);

    info!("example finished!");
}
