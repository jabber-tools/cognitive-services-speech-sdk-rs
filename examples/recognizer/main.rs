mod continuous_recognition_from_file;
mod continuous_recognition_pull_stream;
mod continuous_recognition_push_stream;
mod embedded_recognize_once_async_from_file;
mod from_microphone;
mod helpers;
mod recognize_once_async_from_file;
mod recognize_once_async_from_push_stream;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    // for convenience MS subscription key can be put into file read by set_env_vars
    helpers::set_env_vars("/tmp/path/to/subscription/key");
    env_logger::init();

    recognize_once_async_from_file::run_example().await;
    continuous_recognition_from_file::run_example().await;
    continuous_recognition_push_stream::run_example().await;
    recognize_once_async_from_push_stream::run_example().await;
    continuous_recognition_pull_stream::run_example().await;
    // works only on system with properly configured microphone
    // from_microphone::run_example().await;

    // not available in public release yet
    //embedded_recognize_once_async_from_file::run_example().await;
}
