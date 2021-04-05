use cognitive_services_speech_sdk_rs::audio::{AudioConfig, PushAudioInputStream};
use cognitive_services_speech_sdk_rs::dialog::{BotFrameworkConfig, DialogServiceConnector};
use log::*;
use std::env;
use std::io::Read;

/// convenience function to setup environment variables
/// subscription key is taken from external file
pub fn set_env_vars(ms_key_file_path: &str) {
    let msskey: String = std::fs::read_to_string(ms_key_file_path)
        .unwrap()
        .trim()
        .to_owned();

    env::set_var("MSSubscriptionKey", msskey);
    env::set_var("MSServiceRegion", "westeurope");
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
}

pub fn set_callbacks(dialog_svc_connector: &mut DialogServiceConnector) {
    dialog_svc_connector
        .set_session_started_cb(|event| info!(">set_session_started_cb {:?}", event))
        .unwrap();

    dialog_svc_connector
        .set_session_stopped_cb(|event| info!(">set_session_stopped_cb {:?}", event))
        .unwrap();

    dialog_svc_connector
        .set_recognizing_cb(|event| info!(">set_recognizing_cb {:?}", event.result.text))
        .unwrap();

    dialog_svc_connector
        .set_recognized_cb(|event| info!(">set_recognized_cb {:?}", event))
        .unwrap();

    dialog_svc_connector
        .set_canceled_cb(|event| info!(">set_canceled_cb {:?}", event))
        .unwrap();

    dialog_svc_connector
        .set_activity_received_cb(|event| info!(">set_activity_received_cb {:?}", event))
        .unwrap();
}

pub fn push_file_into_stream(filename: &str, mut audio_push_stream: PushAudioInputStream) {
    let mut file = std::fs::File::open(filename).unwrap();
    let chunk_size = 1000;

    loop {
        // info!("pushing");
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = file
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)
            .unwrap();
        if n == 0 {
            break;
        }
        audio_push_stream.write(chunk).unwrap();
        if n < chunk_size {
            break;
        }
    }

    audio_push_stream.close_stream().unwrap();
}

/// retrieves full path of file with filename
/// from examples/sample_files folder
pub fn get_sample_file(filename: &str) -> String {
    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("examples");
    dir.push("sample_files");
    dir.push(filename);
    dir.into_os_string().into_string().unwrap()
}

pub fn dialog_service_connector_from_audio_cfg(
    audio_config: AudioConfig,
) -> DialogServiceConnector {
    let bot_fw_config = BotFrameworkConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let dialog_service_connector =
        DialogServiceConnector::from_config(bot_fw_config, Some(audio_config)).unwrap();
    dialog_service_connector
}
