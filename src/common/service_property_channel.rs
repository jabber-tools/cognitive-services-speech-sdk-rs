/// ServicePropertyChannel defines channels used to pass property settings to service.
#[derive(Debug)]
pub enum ServicePropertyChannel {
    /// URIQueryParameter uses URI query parameter to pass property settings to service.
    URIQueryParameter = 0,
}

impl From<ServicePropertyChannel> for u32 {
    fn from(channel: ServicePropertyChannel) -> Self {
        channel as u32
    }
}

impl From<ServicePropertyChannel> for i32 {
    fn from(channel: ServicePropertyChannel) -> Self {
        channel as i32
    }
}
