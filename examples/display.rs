use std::time::Duration;
use ege::app::App;
use ege::window::color::RGBColor;
use ege::window::x11::X11Window;

extern crate x11;

fn window_handler(_app: &App, window: &X11Window) {
    window.draw_rect(RGBColor(255, 0, 0), 30, 30, 100, 50);
    window.draw_circ(&RGBColor(0, 255, 0), 50, 50, 10);
}

fn main() {
    let mut app = App::new("display example");

    app.add_delay(Duration::from_millis(20));
    app.add_window(500, 200);
    app.add_window_handler(window_handler);

    app.start();
}
