use cognitive_services_speech_sdk_rs::audio::stream_format::get_wave_format_pcm;
use log::*;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("calling get_wave_format_pcm");
    let wave_format = get_wave_format_pcm(16000, None, None).unwrap();
    info!("called get_wave_format_pcm {:?}", wave_format);
}
