use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{event::GameOver, AppState};

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

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        let new_state = match simulation_state.0 {
            AppState::MainMenu => commands.insert_resource(NextState(Some(AppState::Game))),
            _ => (),
        };
    }
}

pub fn transition_to_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        let new_state = match simulation_state.0 {
            AppState::Game => commands.insert_resource(NextState(Some(AppState::MainMenu))),
            _ => (),
        };
    }
}
