mod helpers;
mod listen_once_async;
mod start_keyword_recognition_async;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    helpers::set_env_vars("/home/adambe/projects/mskey-msspeech");
    env_logger::init();

    listen_once_async::run_example().await;
    start_keyword_recognition_async::run_example().await;
}
