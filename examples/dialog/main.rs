mod helpers;
mod listen_once_async;
mod start_keyword_recognition_async;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    // for convenience MS subscription key can be put into file read by set_env_vars
    helpers::set_env_vars("/tmp/path/to/subscription/key-speech");
    env_logger::init();

    listen_once_async::run_example().await;
    start_keyword_recognition_async::run_example().await;
}
