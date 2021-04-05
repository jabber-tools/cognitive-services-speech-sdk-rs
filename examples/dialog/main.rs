mod from_push_audio_stream;
mod helpers;

#[tokio::main]
async fn main() {
    helpers::set_env_vars("/home/adambe/projects/mskey-msspeech");
    env_logger::init();

    // uncoment one of the lines below to run concrete recognizer example
    from_push_audio_stream::run_example().await;
}
