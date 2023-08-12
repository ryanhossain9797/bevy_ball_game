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

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
        app_state_next_state.set(AppState::GameOver);
        println!("Entered AppState::GameOver");
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<AppState>>
) {
    match keyboard_input.just_pressed(KeyCode::P) {
        true => {
            match simulation_state.0 {
                AppState::MainMenu => simulation_state_next_state.set(AppState::Game),
                _ => (),
            };
        }
        false => (),
    }
}

pub fn transition_to_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<AppState>>
) {
    match keyboard_input.just_pressed(KeyCode::P) {
        true => {
            match simulation_state.0 {
                AppState::Game => simulation_state_next_state.set(AppState::MainMenu),
                _ => (),
            };
        }
        false => (),
    }
}
