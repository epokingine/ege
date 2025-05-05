pub mod keyboard;
pub mod mouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    Empty,
    /// All keys down
    KeyboardKeys,
    /// Single key up
    KeyboardUp,
    /// Single key down
    KeyboardDown,
    /// Any mouse event
    Mouse,
    /// Mouse movement
    MouseMove,
    /// Mouse button up
    MouseUp,
    /// Mouse button down
    MouseDown,
}
