use super::helpers;
use log::*;

// uses audio output PULL stream but syntehtized bytes
/// are read gradually during synthetis via read method
/// provided by audio output pull stream
#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running speak_text_async_2 example...");
    info!("---------------------------------------------------");

    let (mut speech_synthesizer, pull_stream) = helpers::speech_synthesizer();

    helpers::set_callbacks(&mut speech_synthesizer);

    let _ = tokio::spawn(async move {
        if let Err(err) = speech_synthesizer
            .speak_text_async("This is sample text to transcribe")
            .await
        {
            error!("speak_text_async error {:?}", err);
        }
    });

    let mut final_audio_data: Vec<u8> = vec![];
    loop {
        // when reading smaller chunks (e.g. 100) we are actually missing data
        // and recognition will not succeed. output pull stream probably implements
        // some kind of ring buffer which needs to be pulled/read before it gets
        // replaced with new values
        let mut audio_data_chunk = pull_stream.read(1024).unwrap();

        // quick & dirty way how to find out if we got vector of zeroes
        let audio_data_chunk_no0 = audio_data_chunk
            .clone()
            .into_iter()
            .filter(|&i| i == 0)
            .collect::<Vec<_>>();

        info!("data read: {:?}", audio_data_chunk);
        if audio_data_chunk.len() == 0 || audio_data_chunk_no0.len() == 0 {
            info!("no more data to read!");
            break;
        }
        final_audio_data.append(&mut audio_data_chunk);
    }
    // info!("final_audio_data {:?}", final_audio_data);
    let recognition_result = helpers::recognize_synthetis_result(final_audio_data).await;
    info!("recognition_result {:?}", recognition_result);

    info!("example finished!");
}
