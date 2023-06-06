use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod component;
mod entity;
mod system;

mod keyboard_helper;

use entity::star::STAR_SPAWN_TIME;
use rand::*;
use system::enemy::*;
use system::player::*;
use system::score::*;
use system::star::*;

fn main() {
    App::new()
        .init_resource::<StarTimer>()
        .init_resource::<EnemyTimer>()
        .init_resource::<Score>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_initial_enemies)
        .add_startup_system(spawn_initial_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(bounce_enemy_movement)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_pickup_star)
        .add_system(display_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_occassional_stars)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_occassional_enemies)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
