mod activity_received_event;
mod dialog_service_connector;

// re-export structs directly under dialog module
pub use self::activity_received_event::ActivityReceivedEvent;
pub use self::dialog_service_connector::DialogServiceConnector;
