#[derive(Debug)]
pub enum CancellationErrorCode {
    NoError = 0,
    AuthenticationFailure = 1,
    BadRequest = 2,
    TooManyRequests = 3,
    Forbidden = 4,
    ConnectionFailure = 5,
    ServiceTimeout = 6,
    ServiceError = 7,
    ServiceUnavailable = 8,
    RuntimeError = 9,
}

impl CancellationErrorCode {
    pub fn from_u32(code: u32) -> Self {
        return match code {
            0 => CancellationErrorCode::NoError,
            1 => CancellationErrorCode::AuthenticationFailure,
            2 => CancellationErrorCode::BadRequest,
            3 => CancellationErrorCode::TooManyRequests,
            4 => CancellationErrorCode::Forbidden,
            5 => CancellationErrorCode::ConnectionFailure,
            6 => CancellationErrorCode::ServiceTimeout,
            7 => CancellationErrorCode::ServiceError,
            8 => CancellationErrorCode::ServiceUnavailable,
            _ => CancellationErrorCode::RuntimeError,
        };
    }
}
