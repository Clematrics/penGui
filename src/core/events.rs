use std::collections::HashSet;

/// Describe a button of a mouse
/// Only the three most important are implemented
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    // TODO: add other buttons
}

/// Describe a key on a keyboard
/// Not all keys are implemented
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

/// An event that can be propagated through a UI
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
    // Focus
    FocusForward,
    FocusBackward,
}

/// The response a widget can give after receiving an event.
/// This will determine if the event will continue to be
/// propagated or not.
///
/// If the response is
/// - `Registered`, the event won't be propagated to the widgets behind
/// - `PassivelyRegistered`, it indicates the widget did something
/// with the event but allows it to be propagated further
/// - `Pass`, the widget did nothing with the event and it will be propagated to
/// the widgets behind
pub enum EventResponse {
    Registered,
    PassivelyRegistered,
    Pass,
}

/// Stores the current state of some input methods
pub struct InputState {
    pub mouse_position: (f32, f32),
    pub mouse_movement: (f32, f32),
    pub mouse_wheel_movement: f32,
    pub mouse_buttons_pressed: HashSet<MouseButton>,
    pub mouse_inside_window: bool,
    pub keys_pressed: HashSet<Key>,
}

impl InputState {
    /// Modifies the state according to an event
    pub fn event(&mut self, event: &Event) {
        match event {
            // Mouse events
            Event::MouseButtonPressed(button) => {
                self.mouse_buttons_pressed.insert(*button);
            }
            Event::MouseButtonReleased(button) => {
                self.mouse_buttons_pressed.remove(button);
            }
            Event::MouseScrolled(delta) => self.mouse_wheel_movement = *delta,
            Event::MouseEntered => self.mouse_inside_window = true,
            Event::MouseLeft => self.mouse_inside_window = false,
            Event::MouseMoved(x, y) => {
                let (px, py) = self.mouse_position;
                let (dx, dy) = (px - *x, py - *y);
                self.mouse_position = (*x, *y);
                self.mouse_movement = (dx, dy);
            }
            // Keyboard events
            Event::KeyPressed(key) => {
                self.keys_pressed.insert(*key);
            }
            Event::KeyReleased(key) => {
                self.keys_pressed.remove(key);
            }
            // Other events
            _ => (),
        }
    }
}

impl Default for InputState {
    /// Default state
    fn default() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            mouse_movement: (0.0, 0.0),
            mouse_wheel_movement: 0.0,
            mouse_buttons_pressed: HashSet::new(),
            mouse_inside_window: false,
            keys_pressed: HashSet::new(),
        }
    }
}
