use std::result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventError {
    WrongEventType,
    IdenticalEventTypes,
    SetupEventFailed,
}

pub type Result<T> = result::Result<T, EventError>;
