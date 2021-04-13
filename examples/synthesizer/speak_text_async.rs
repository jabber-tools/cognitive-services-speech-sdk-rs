use super::helpers;
use log::*;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running speak_text_async example...");
    info!("---------------------------------------------------");

    let text = "This is sample text to transcribe";

    let mut speech_synthesize = helpers::speech_synthesizer();

    helpers::set_callbacks(&mut speech_synthesize);

    match speech_synthesize.speak_text_async(text).await {
        Err(err) => error!("speak_text_async error {:?}", err),
        Ok(result) => {
            info!("got result!");
            let recognition_result = helpers::recognize_synthetis_result(result.audio_data).await;
            info!("recognition_result {:?}", recognition_result);
        }
    }

    sleep(Duration::from_millis(10000)).await;

    info!("example finished!");
}
