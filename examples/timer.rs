use std::time::Duration;

use ege::{
    app::App,
    events::{handler::EventHandler, Event},
};

fn once_every_2_secs(_app: &mut App) {
    println!("2 seconds have passed");
}

fn main() {
    let mut app = App::new("time");
    app.add_event_handler(EventHandler {
        event_type: Event::TimedEvent,
        interval: Some(Duration::new(2, 0)),
        handler_fn: once_every_2_secs,
    })
    .unwrap();

    app.start();
}
