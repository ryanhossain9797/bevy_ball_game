use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod component;
mod event;
pub mod resource;
mod system;

mod keyboard_helper;

use component::star::STAR_SPAWN_TIME;
use event::*;
use rand::*;
use resource::EnemyTimer;
use resource::HighScore;
use resource::Score;
use resource::StarTimer;
use system::enemy::*;
use system::player::*;
use system::star::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<StarTimer>()
        .init_resource::<EnemyTimer>()
        .init_resource::<Score>()
        .init_resource::<HighScore>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_initial_enemies)
        .add_startup_system(spawn_initial_stars)
        .add_system(exit_game)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(bounce_enemy_movement)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_pickup_star)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_occassional_stars)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_occassional_enemies)
        .add_system(exit_game)
        .add_system(handle_game_over)
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

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScore>,
) {
    if !game_over_event_reader.is_empty() {
        game_over_event_reader.iter().for_each(|event| {
            high_scores
                .high_score
                .push(("Player".to_string(), event.score));
            println!("{}", event.score)
        });
    }
}
