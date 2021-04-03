mod continuous_recognition_from_file;
mod continuous_recognition_push_stream;
mod continuous_recognition_push_stream_once;
mod from_microphone;
mod helpers;
mod recognize_once;

#[tokio::main]
async fn main() {
    helpers::set_env_vars("/home/adambe/projects/mskey");
    env_logger::init();

    // uncoment one of the lines below to run concrete recognizer example
    recognize_once::run_example().await;
    continuous_recognition_from_file::run_example().await;
    continuous_recognition_push_stream::run_example().await;
    continuous_recognition_push_stream_once::run_example().await;
    // from_microphone::run_example().await;
}
