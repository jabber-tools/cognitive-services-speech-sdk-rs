/// StreamStatus defines the possible status of audio data stream.
#[derive(Debug)]
pub enum StreamStatus {
    /// StreamStatusUnknown indicates the audio data stream status is unknown.
    StreamStatusUnknown = 0,

    /// StreamStatusNoData indicates that the audio data stream contains no data.
    StreamStatusNoData = 1,

    /// StreamStatusPartialData indicates that the audio data stream contains partial data of a speak request.
    StreamStatusPartialData = 2,

    /// StreamStatusAllData indicates the audio data stream contains all data of a speak request.
    StreamStatusAllData = 3,

    /// StreamStatusCanceled indicates the audio data stream was canceled.
    StreamStatusCanceled = 4,
}

impl StreamStatus {
    pub fn from_u32(status: u32) -> Self {
        match status {
            0 => StreamStatus::StreamStatusUnknown,
            1 => StreamStatus::StreamStatusNoData,
            2 => StreamStatus::StreamStatusPartialData,
            3 => StreamStatus::StreamStatusAllData,
            _ => StreamStatus::StreamStatusCanceled,
        }
    }
}

impl From<u32> for StreamStatus {
    fn from(value: u32) -> Self {
        StreamStatus::from_u32(value)
    }
}

impl From<i32> for StreamStatus {
    fn from(value: i32) -> Self {
        StreamStatus::from_u32(value as u32)
    }
}
