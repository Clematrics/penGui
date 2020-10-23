pub enum UserEvent {
    MouseEvent(MouseEvent),
    KeyPress(Key),
}

pub enum MouseEvent {
    LeftClick(f32, f32),
    RightClick(f32, f32),
    MiddleClick(f32, f32),
}

pub enum Key {
    A,
    B,
}
