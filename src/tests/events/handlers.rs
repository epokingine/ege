use std::time::Duration;

use crate::{
    app::App,
    events::{handler::EventHandler, Event},
};

fn empty1(_app: &mut App) {}

fn empty2(_app: &mut App) {}

#[test]
fn test_equal_handlers() {
    let handler1 = EventHandler {
        event_type: Event::Loop,
        interval: None,
        handler_fn: empty1,
    };
    let handler2 = EventHandler {
        event_type: Event::Loop,
        interval: None,
        handler_fn: empty1,
    };
    let handler3 = EventHandler {
        event_type: Event::Loop,
        interval: None,
        handler_fn: empty2,
    };

    assert_eq!(handler1, handler2);
    assert_ne!(handler1, handler3);
}

#[test]
fn test_sort_handlers() {
    let mut handlers = vec![
        EventHandler {
            event_type: Event::Input,
            interval: None,
            handler_fn: empty1,
        },
        EventHandler {
            event_type: Event::TimedEvent,
            interval: Some(Duration::from_millis(274)),
            handler_fn: empty2,
        },
        EventHandler {
            event_type: Event::Shutdown,
            interval: None,
            handler_fn: empty2,
        },
        EventHandler {
            event_type: Event::TimedEvent,
            interval: Some(Duration::from_millis(12)),
            handler_fn: empty1,
        },
    ];

    handlers.sort();

    assert_eq!(handlers[3].interval, None);
    assert_eq!(handlers[2].interval, None);
    assert_eq!(handlers[1].interval, Some(Duration::from_millis(274)));
    assert_eq!(handlers[0].interval, Some(Duration::from_millis(12)));
}
