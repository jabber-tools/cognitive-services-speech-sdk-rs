use super::helpers;
use log::*;

#[allow(dead_code)]
pub async fn run_example() {
    info!("running continuous_recognition_push_stream_once example...");

    let filename = helpers::get_sample_file("whats_the_weather_like.wav");

    let (mut speech_recognizer, audio_push_stream) = helpers::speech_recognizer_from_push_stream();
    helpers::set_callbacks(&mut speech_recognizer);
    helpers::push_file_into_stream(&filename, audio_push_stream);
    let speech_reco_res = speech_recognizer.recognize_once_async().await;

    info!("speech_reco_res {:#?}", speech_reco_res);

    info!("example finished!");
}
