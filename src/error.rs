use std::error::Error;

#[derive(Debug)]
pub enum OneTeslaError {
    NoCompatibleDeviceFound,
    SystemHasNoMidiSupport,
    CannotConnectToDevice,
    FailedToSendMessageToDevice,
}

impl Error for OneTeslaError {
    fn description(&self) -> &str {
        match *self {
            OneTeslaError::NoCompatibleDeviceFound => "No compatible device was found!",
            OneTeslaError::SystemHasNoMidiSupport => "Your system has no midi support!",
            OneTeslaError::CannotConnectToDevice => "Cannot connect to the given device!",
            OneTeslaError::FailedToSendMessageToDevice => "Could not send message to device!",
        }
    }
}

impl std::fmt::Display for OneTeslaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            _ => write!(f, "{}", self.description())
        }
    }
}
