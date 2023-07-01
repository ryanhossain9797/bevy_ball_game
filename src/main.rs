use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod enemy;
mod event;
pub mod player;
pub mod score;
pub mod star;

mod keyboard_helper;

use enemy::resource::EnemyTimer;
use enemy::*;
use event::*;
use player::*;
use rand::*;
use score::*;
use star::resource::StarTimer;
use star::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(spawn_camera)
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

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    if !game_over_event_reader.is_empty() {
        game_over_event_reader
            .iter()
            .for_each(|event| println!("Your score is {}", event.score));
    }
}
