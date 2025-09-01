use log::error;

// TODO: Allow developer to decide what windowing system will be used, for now it will only be X11 since both Linux and
// Mac supports it
/// An interface for an X11 window
pub mod x11;

pub mod color;
pub mod image;

pub fn window_error() {
    error!("An error occured with the windowing system");
}
