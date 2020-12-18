pub enum MouseButton {
    Left,
    Middle,
    Right,
    // TODO: add other buttons
}

pub enum Key {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Backspace,
    Delete,
    Escape,
    Return,
    Space,
    Tab,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Numpad0,
    NumpadAdd,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    // TODO: add other keys
}

pub enum Event {
    // Mouse events
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseScrolled(f32),
    MouseEntered,
    MouseLeft,
    MouseMoved(f32, f32),
    // Keyboard events
    KeyPressed(Key),
    KeyReleased(Key),
    // Text
    Character(char),
}

use std::collections::HashSet;

pub struct InputState {
    mouse_position: (f32, f32),
    mouse_movement: (f32, f32),
    mouse_buttons_pressed: HashSet<MouseButton>,
    keys_pressed: HashSet<Key>,
    caps_lock_activated: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            mouse_movement: (0.0, 0.0),
            mouse_buttons_pressed: HashSet::new(),
            keys_pressed: HashSet::new(),
            caps_lock_activated: false,
        }
    }
}
