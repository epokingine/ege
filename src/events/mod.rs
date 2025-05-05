use crate::app::App;

pub mod errors;
pub mod handler;
pub mod handlers;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    Setup,
    Loop,
    TimedEvent,
    Input,
    Condition(fn(&mut App) -> bool),
    Shutdown,
}
