use crate::{
    app::App,
    events::{handler::EventHandler, Event},
};

fn test_condition1(_app: &mut App) -> bool {
    5 == 5
}

fn condition1_event(_app: &mut App) {
    //println!("5 == 5");
}

fn test_condition2(app: &mut App) -> bool {
    app.elapsed_time().unwrap().as_millis() >= 500
}

fn condition2_event(app: &mut App) {
    //println!("Stop!");
    app.stop();
}

#[test]
fn test_condition() {
    let mut app = App::new("conditions");

    app.add_event_handler(EventHandler {
        event_type: Event::Condition(test_condition1),
        interval: None,
        handler_fn: condition1_event,
    })
    .expect("Failed");

    app.add_event_handler(EventHandler {
        event_type: Event::Condition(test_condition2),
        interval: None,
        handler_fn: condition2_event,
    })
    .expect("Failed");

    app.start();
}
