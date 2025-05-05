use ege::{
    app::App,
    events::{handler::EventHandler, Event},
};

fn simple_setup(_app: &mut App) {
    println!("A simple setup function");
}

fn goodbye(_app: &mut App) {
    println!("See ya next time!");
}

fn main() {
    let mut app = App::new("basic app");
    app.add_event_handler(EventHandler {
        event_type: Event::Setup,
        interval: None,
        handler_fn: simple_setup,
    })
    .expect("Failed to add");

    app.add_event_handler(EventHandler {
        event_type: Event::Shutdown,
        interval: None,
        handler_fn: goodbye,
    })
    .expect("Failed to add");

    app.start();
}
