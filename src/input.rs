
use ggez::event::*;
use ggez_goodies::input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Fire,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    Vert,
    Horz,
}


pub fn create_input_manager() -> input::InputManager<Axis, Button> {
    input::InputManager::new()
        .bind_key_to_axis(Keycode::Up, Axis::Vert, true)
        .bind_key_to_axis(Keycode::Down, Axis::Vert, false)
        .bind_key_to_axis(Keycode::Left, Axis::Horz, false)
        .bind_key_to_axis(Keycode::Right, Axis::Horz, true)
        .bind_key_to_button(Keycode::Z, Button::Fire)
}
