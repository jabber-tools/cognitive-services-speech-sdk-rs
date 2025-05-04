use std::env;

use cognitive_services_speech_sdk_rs::{
    common::{SpeechSynthesisOutputFormat, StreamStatus},
    speech::{AudioDataStream, SpeechConfig, SpeechSynthesisRequest, SpeechSynthesizer},
};
use log::{error, info};
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};

#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running input_text_streaming example...");
    info!("---------------------------------------------------");

    // Initialize audio output system
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to create audio output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

    // Use input text streaming to lower speech synthesis latency.
    // https://learn.microsoft.com/en-us/azure/ai-services/speech-service/how-to-lower-speech-synthesis-latency?pivots=programming-language-cpp#input-text-streaming
    let region = env::var("MSServiceRegion").unwrap();
    // To use the text stream API, you have to use the websocket V2 endpoint.
    let tts_endpoint =
        format!("wss://{region}.tts.speech.microsoft.com/cognitiveservices/websocket/v2");
    let subscription_key = env::var("MSSubscriptionKey").unwrap();

    let mut speech_config =
        SpeechConfig::from_endpoint_with_subscription(tts_endpoint, subscription_key.to_string())
            .unwrap();
    speech_config
        .set_get_speech_synthesis_language("en-US".to_string())
        .unwrap();
    speech_config
        .set_get_speech_synthesis_voice_name("en-US-AvaMultilingualNeural".to_string())
        .unwrap();

    speech_config
        .set_speech_synthesis_output_format(SpeechSynthesisOutputFormat::Raw24Khz16BitMonoPcm)
        .unwrap();

    // Set the audio_config parameter to None to control the audio play ourselves.
    let speech_synthesizer =
        SpeechSynthesizer::from_optional_audio_config(speech_config, None).unwrap();

    let request = SpeechSynthesisRequest::new_text_streaming_request().unwrap();

    match speech_synthesizer.start_speaking_async(&request).await {
        Err(err) => {
            error!("speak_text_async error {:?}", err);
        }
        Ok(result) => {
            let audio_stream = AudioDataStream::from_speech_synthesis_result(result).unwrap();
            let buffer = &mut [0u8; 32000]; // Buffer to receive audio data

            std::thread::spawn(move || {
                for response in AiResponse::new() {
                    info!("++++++ Writing input text stream: {}", response);
                    request.send_text_piece(response).unwrap();
                }
                request.finish_input().unwrap();
                info!("====== Input text stream closed");
            });

            loop {
                let status = audio_stream.get_status().unwrap();
                match status {
                    StreamStatus::StreamStatusNoData => {
                        // Sleep for a short duration to avoid busy waiting
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }
                    StreamStatus::StreamStatusCanceled => {
                        info!("audio data stream was canceled");
                        break;
                    }
                    StreamStatus::StreamStatusAllData => {
                        info!("audio data stream is finished");
                        break;
                    }
                    StreamStatus::StreamStatusPartialData => {
                        info!("partical data received from audio data stream");

                        let read_size = audio_stream.read(buffer).unwrap();

                        if read_size > 0 {
                            // Create a buffer with only the data we actually read
                            let audio_data = buffer[..read_size as usize].to_vec();

                            if !audio_data.is_empty() {
                                // Convert the audio data to i16 samples
                                let samples = audio_data
                                    .chunks_exact(2)
                                    .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                                    .collect::<Vec<i16>>();
                                let sample_buffer = SamplesBuffer::new(1, 24000, samples);

                                // Play the audio chunk
                                sink.append(sample_buffer);
                                sink.play();
                            }
                        }
                    }
                    StreamStatus::StreamStatusUnknown => {
                        info!("audio data stream status is unknown");
                        break;
                    }
                }
            }

            // Wait for all audio to finish playing before exiting
            {
                sink.sleep_until_end();
            }
        }
    }

    info!("example finished!");
}

// This is a mock implementation of an AI response generator.
struct AiResponse {
    responses: Vec<String>,
    current_index: usize,
}

impl AiResponse {
    pub fn new() -> Self {
        let responses = [
            "Input text streaming.",
            "Text streaming allows real-time text processing for rapid audio generation. ",
            "It's perfect for dynamic text vocalization, ",
            "such as reading outputs from AI models like GPT in real-time. ",
            "This feature minimizes latency ",
            "and improves the fluidity and responsiveness of audio outputs, ",
            "making it ideal for interactive applications, ",
            "live events, ",
            "and responsive AI-driven dialogues.",
        ];
        AiResponse {
            responses: responses.map(|x| x.to_string()).to_vec(),
            current_index: 0,
        }
    }
}

impl Iterator for AiResponse {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.current_index < self.responses.len() {
            // Simulate a delay for the AI response
            std::thread::sleep(std::time::Duration::from_secs(2));
            let response = self.responses[self.current_index].clone();
            self.current_index += 1;
            Some(response)
        } else {
            None
        }
    }
}
