/// CancellationReason defines the possible reasons a recognition result might be canceled.
#[derive(Debug)]
pub enum CancellationReason {
    /// Indicates that an error occurred during speech recognition.
    Error = 1,
    /// Indicates that the end of the audio stream was reached.
    EndOfStream = 2,
}

impl CancellationReason {
    pub fn from_u32(code: u32) -> Self {
        return match code {
            1 => CancellationReason::Error,
            _ => CancellationReason::EndOfStream,
        };
    }
}
