//! Package common contains the definitions for many of the shared objects and properties in the Speech SDK.
mod cancellation_error_code;
mod cancellation_reason;
mod output_format;
mod profanity_option;
mod property_collection;
mod property_id;
mod result_reason;
mod service_property_channel;
mod speech_synthesis_output_format;
mod stream_status;
mod synthesis_voice_type;

// re-export structs directly under common module
pub use self::cancellation_error_code::CancellationErrorCode;
pub use self::cancellation_reason::CancellationReason;
pub use self::output_format::OutputFormat;
pub use self::profanity_option::ProfanityOption;
pub use self::property_collection::PropertyCollection;
pub use self::property_id::PropertyId;
pub use self::result_reason::ResultReason;
pub use self::service_property_channel::ServicePropertyChannel;
pub use self::speech_synthesis_output_format::SpeechSynthesisOutputFormat;
pub use self::stream_status::StreamStatus;
pub use self::synthesis_voice_type::SynthesisVoiceType;
