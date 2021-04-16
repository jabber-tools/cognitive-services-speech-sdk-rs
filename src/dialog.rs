//! Package dialog provides functionality for creating custom voice assistant applications and managing the related interaction flow.
mod activity_received_event;
mod bot_framework_config;
mod custom_commands_config;
mod dialog_service_config;
mod dialog_service_connector;

// re-export structs directly under dialog module
pub use self::activity_received_event::ActivityReceivedEvent;
pub use self::bot_framework_config::BotFrameworkConfig;
pub use self::custom_commands_config::CustomCommandsConfig;
pub use self::dialog_service_config::DialogServiceConfig;
pub use self::dialog_service_connector::DialogServiceConnector;
