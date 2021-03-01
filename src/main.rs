use cognitive_services_speech_sdk_rs::audio::{
    input_stream::create_push_stream_from_format, stream_format::get_wave_format_pcm,
};
use cognitive_services_speech_sdk_rs::config::AudioConfig;
use log::*;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("calling get_wave_format_pcm");
    let wave_format = get_wave_format_pcm(16000, None, None).unwrap();
    info!("called get_wave_format_pcm {:?}", wave_format);

    info!("calling create_push_stream_from_format");
    let push_stream = create_push_stream_from_format(wave_format).unwrap();
    info!("called create_push_stream_from_format {:?}", push_stream);

    info!("calling from_stream_input");
    let audio_config = AudioConfig::from_stream_input(push_stream).unwrap();
    info!("called from_stream_input {:?}", audio_config);
}
