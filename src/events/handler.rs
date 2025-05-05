use std::time::Duration;

use crate::app::App;

use super::Event;

/// Structure for holding specific event handler
#[derive(Debug)]
pub struct EventHandler {
    pub event_type: Event,
    pub interval: Option<Duration>,
    pub handler_fn: fn(&mut App), // This may need to change for FFI reasons
}

impl EventHandler {
    pub fn new(event_type: Event, interval: Option<Duration>, handler_fn: fn(&mut App)) -> Self {
        return EventHandler {
            event_type,
            interval,
            handler_fn,
        };
    }
}

/// Only for checking handler functions
impl PartialEq for EventHandler {
    fn eq(&self, other: &Self) -> bool {
        let self_handler_fn = self.handler_fn as *const ();
        let other_handler_fn = other.handler_fn as *const ();
        self.event_type == other.event_type && self_handler_fn == other_handler_fn
    }
}

impl Eq for EventHandler {}

/// Only for checking durations on timed events
impl Ord for EventHandler {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // If an array of event handlers is being sorted by duration, any duration that is ``None`` should be moved to
        // the end.
        if self.interval.is_none() {
            return std::cmp::Ordering::Greater;
        } else if other.interval.is_none() {
            return std::cmp::Ordering::Less;
        }

        self.interval
            .unwrap()
            .cmp(&other.interval.unwrap())
            .then(self.interval.unwrap().cmp(&other.interval.unwrap()))
    }
}

impl PartialOrd for EventHandler {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
