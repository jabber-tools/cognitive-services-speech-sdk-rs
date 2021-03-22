#[derive(Debug)]
pub enum CancellationReason {
    Error = 1,
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
