use bevy::prelude::*;

pub enum KeyboardDirection {
    Up,
    Right,
    Down,
    Left,
}

pub fn direction(keyboard_input: &Res<Input<KeyCode>>) -> Vec<KeyboardDirection> {
    let mut directions = Vec::new();

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        directions.push(KeyboardDirection::Up)
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        directions.push(KeyboardDirection::Right)
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        directions.push(KeyboardDirection::Down)
    }
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        directions.push(KeyboardDirection::Left)
    }

    directions
}
