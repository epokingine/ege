use std::time::Duration;

use crate::{
    app::App,
    events::{handler::EventHandler, Event},
};

static mut TWO_SECONDS: bool = false;

fn once_every_2_secs(app: &mut App) {
    println!("running this thing");
    unsafe {
        if TWO_SECONDS == true {
            app.stop()
        } else {
            TWO_SECONDS = true;
        }
    }
}

#[test]
fn test_timed_events() {
    let mut app = App::new("time");
    app.add_event_handler(EventHandler {
        event_type: Event::TimedEvent,
        interval: Some(Duration::new(2, 0)),
        handler_fn: once_every_2_secs,
    })
    .unwrap();

    app.start();
    assert_eq!(unsafe { TWO_SECONDS }, true);
}
