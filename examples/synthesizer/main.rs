mod helpers;
mod speak_text_async;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    helpers::set_env_vars("/home/adambe/projects/mskey");
    env_logger::init();

    speak_text_async::run_example().await;
}
