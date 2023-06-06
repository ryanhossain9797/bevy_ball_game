use crate::*;
use bevy::prelude::*;
use entity::player::*;

use crate::keyboard_helper::KeyboardDirection;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let (x, y) = keyboard_helper::direction(&keyboard_input).iter().fold(
            (0.0, 0.0),
            |(x, y), direction| match direction {
                KeyboardDirection::Up => (x, y + 1.0),
                KeyboardDirection::Right => (x + 1.0, y),
                KeyboardDirection::Down => (x, y - 1.0),
                KeyboardDirection::Left => (x - 1.0, y),
            },
        );

        let direction = Vec3::new(x, y, 0.0).normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query
            .get_single()
            .expect("Primary windows not found");

        let mut translation = player_transform.translation;
        {
            let x_min = 0.0 + HALF_PLAYER_SIZE;
            let x_max = window.width() - HALF_PLAYER_SIZE;
            if translation.x < x_min {
                translation.x = x_min;
            } else if translation.x > x_max {
                translation.x = x_max;
            }
        }
        {
            let y_min = 0.0 + HALF_PLAYER_SIZE;
            let y_max = window.height() - HALF_PLAYER_SIZE;

            if translation.y < y_min {
                translation.y = y_min;
            } else if translation.y > y_max {
                translation.y = y_max;
            }
        }
        player_transform.translation = translation;
    }
}
