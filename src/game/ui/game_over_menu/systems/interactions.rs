use bevy::{app::AppExit, prelude::*};

use crate::{
    game::ui::game_over_menu::{
        components::{MainMenuButton, QuitButton, RestartButton},
        styles::{MENU_HOVERED_BUTTON_COLOR, MENU_NORMAL_BUTTON_COLOR, MENU_PRESSED_BUTTON_COLOR},
    },
    AppState,
};

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    match button_query.get_single_mut() {
        Ok((interaction, mut background_color)) => {
            let (new_color, go_to_main_menu) = match *interaction {
                Interaction::Hovered => (MENU_HOVERED_BUTTON_COLOR, false),
                Interaction::Clicked => (MENU_PRESSED_BUTTON_COLOR, true),
                Interaction::None => (MENU_NORMAL_BUTTON_COLOR, false),
            };

            (*background_color) = new_color.into();

            match go_to_main_menu {
                true => app_state_next_state.set(AppState::MainMenu),
                false => (),
            }
        }

        Err(_) => {
            ();
        }
    }
}
pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    match button_query.get_single_mut() {
        Ok((interaction, mut background_color)) => {
            let (new_color, should_quit) = match *interaction {
                Interaction::Hovered => (MENU_HOVERED_BUTTON_COLOR, false),
                Interaction::Clicked => (MENU_PRESSED_BUTTON_COLOR, true),
                Interaction::None => (MENU_NORMAL_BUTTON_COLOR, false),
            };
            (*background_color) = new_color.into();
            match should_quit {
                true => exit_event_writer.send(AppExit),
                false => (),
            }
        }
        Err(_) => {
            ();
        }
    }
}
pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    match button_query.get_single_mut() {
        Ok((interaction, mut background_color)) => {
            let (new_color, should_restart) = match *interaction {
                Interaction::Hovered => (MENU_HOVERED_BUTTON_COLOR, false),
                Interaction::Clicked => (MENU_PRESSED_BUTTON_COLOR, true),
                Interaction::None => (MENU_NORMAL_BUTTON_COLOR, false),
            };
            (*background_color) = new_color.into();
            match should_restart {
                true => app_state_next_state.set(AppState::Game),
                false => (),
            }
        }
        Err(_) => {
            ();
        }
    }
}
