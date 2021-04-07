mod continuous_recognition_from_file;
mod continuous_recognition_pull_stream;
mod continuous_recognition_push_stream;
mod from_microphone;
mod helpers;
mod recognize_once_async_from_file;
mod recognize_once_async_from_push_stream;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    helpers::set_env_vars("/home/adambe/projects/mskey");
    env_logger::init();

    //recognize_once_async_from_file::run_example().await;
    //continuous_recognition_from_file::run_example().await;
    //continuous_recognition_push_stream::run_example().await;
    //recognize_once_async_from_push_stream::run_example().await;
    continuous_recognition_pull_stream::run_example().await;
    // works only on system with properly configured microphone
    // from_microphone::run_example().await;
}
