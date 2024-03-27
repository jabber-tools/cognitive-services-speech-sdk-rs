/// CancellationReason defines the possible reasons a recognition result might be canceled.
#[derive(Debug)]
pub enum CancellationReason {
    /// Indicates that an error occurred during speech recognition.
    Error = 1,
    /// Indicates that the end of the audio stream was reached.
    EndOfStream = 2,
    /// Indicates that request was cancelled by the user.
    CancelledByUser = 3,
}

impl CancellationReason {
    pub fn from_u32(code: u32) -> Self {
        match code {
            1 => CancellationReason::Error,
            2 => CancellationReason::EndOfStream,
            _ => CancellationReason::CancelledByUser,
        }
    }

    pub fn from_i32(code: i32) -> Self {
        CancellationReason::from_u32(code as u32)
    }
}
