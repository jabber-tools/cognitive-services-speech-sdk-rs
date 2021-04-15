use super::helpers;
use log::*;

/// demonstrates how to store synthetized data easily via Audio Data Stream abstraction
#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running audio_data_stream example...");
    info!("---------------------------------------------------");

    let text = "Syntethizing some text here!";

    let (mut speech_synthesize, _) = helpers::speech_synthesizer();

    helpers::set_callbacks(&mut speech_synthesize);

    match speech_synthesize.speak_text_async(text).await {
        Err(err) => error!("speak_text_async error {:?}", err),
        Ok(result) => {
            info!("got result!");
            helpers::save_wav("/tmp/output.wav", result).await;
        }
    }

    info!("example finished!");
}
