use super::helpers;
use log::*;

// tests bookmark and viseme callbacks
#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running speak_ssml_async_bm_viseme example...      ");
    info!("---------------------------------------------------");

    let (mut speech_synthesizer, _) = helpers::speech_synthesizer();

    helpers::set_callbacks_all(&mut speech_synthesizer);
    let text = "<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xmlns:mstts='http://www.w3.org/2001/mstts' xmlns:emo='http://www.w3.org/2009/10/emotionml' xml:lang='en-US'><voice name='en-US-AriaNeural'><mstts:viseme type='redlips_front'>yet</mstts:viseme><bookmark mark='mark'/></voice></speak>";
    match speech_synthesizer.speak_ssml_async(text).await {
        Err(err) => error!("speak_ssml_async error {:?}", err),
        Ok(result) => {
            info!("got result!");
            let recognition_result = helpers::recognize_synthetis_result(result.audio_data).await;
            info!("recognition_result {:?}", recognition_result);
        }
    }

    info!("example finished!");
}
