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

    if let Err(err) = speech_synthesize.speak_text_async(text).await {
        error!("speak_text_async error {:?}", err);
    }
    sleep(Duration::from_millis(10000)).await;

    info!("example finished!");
}
