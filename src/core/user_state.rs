pub struct MouseState {
    pub position: (f64, f64),
    pub left_click: bool,
    pub right_click: bool,
}

pub struct KeyBoardState {}

pub struct UserState {
    pub mouse_state: MouseState,
    pub keyboard_state: KeyBoardState,
}
