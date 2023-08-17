use bevy::{app::AppExit, prelude::*};

use crate::{
    game::{
        ui::pause_menu::{
            components::{MainMenuButton, ResumeButton},
            styles::{
                MENU_HOVERED_BUTTON_COLOR, MENU_NORMAL_BUTTON_COLOR, MENU_PRESSED_BUTTON_COLOR,
            },
        },
        SimulationState,
    },
    AppState,
};

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    match button_query.get_single_mut() {
        Ok((interaction, mut background_color)) => {
            let (new_color, should_resume) = match *interaction {
                Interaction::Hovered => (MENU_HOVERED_BUTTON_COLOR, false),
                Interaction::Clicked => (MENU_PRESSED_BUTTON_COLOR, true),
                Interaction::None => (MENU_NORMAL_BUTTON_COLOR, false),
            };

            (*background_color) = new_color.into();

            match should_resume {
                true => simulation_state_next_state.set(SimulationState::Running),
                false => (),
            }
        }

        Err(_) => {
            ();
        }
    }
}
pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    match button_query.get_single_mut() {
        Ok((interaction, mut background_color)) => {
            let (new_color, should_go_to_main_menu) = match *interaction {
                Interaction::Hovered => (MENU_HOVERED_BUTTON_COLOR, false),
                Interaction::Clicked => (MENU_PRESSED_BUTTON_COLOR, true),
                Interaction::None => (MENU_NORMAL_BUTTON_COLOR, false),
            };
            (*background_color) = new_color.into();
            match should_go_to_main_menu {
                true => app_state_next_state.set(AppState::MainMenu),
                false => (),
            }
        }
        Err(_) => {
            ();
        }
    }
}
