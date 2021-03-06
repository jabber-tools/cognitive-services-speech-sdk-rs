use super::helpers;
use log::*;

/// uses audio output PULL stream but syntehtized bytes
/// are taken from SpeechSynthesisResult returned by speak_text_async
#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running speak_text_async example...");
    info!("---------------------------------------------------");

    let text = "This is sample text to transcribe";

    let (mut speech_synthesizer, _) = helpers::speech_synthesizer();

    helpers::set_callbacks(&mut speech_synthesizer);

    match speech_synthesizer.speak_text_async(text).await {
        Err(err) => error!("speak_text_async error {:?}", err),
        Ok(result) => {
            info!("got result!");
            let recognition_result = helpers::recognize_synthetis_result(result.audio_data).await;
            info!("recognition_result {:?}", recognition_result);
        }
    }

    info!("example finished!");
}
