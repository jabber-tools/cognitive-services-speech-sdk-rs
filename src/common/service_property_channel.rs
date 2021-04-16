/// ServicePropertyChannel defines channels used to pass property settings to service.
#[derive(Debug)]
pub enum ServicePropertyChannel {
    /// URIQueryParameter uses URI query parameter to pass property settings to service.
    URIQueryParameter = 0,
}
