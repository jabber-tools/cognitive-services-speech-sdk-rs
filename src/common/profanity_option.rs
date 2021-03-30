#[derive(Debug)]
pub enum ProfanityOption {
    /// Masked profanity option.
    Masked = 0,

    /// Removed profanity option
    Removed = 1,

    /// Raw profanity option
    Raw = 2,
}
