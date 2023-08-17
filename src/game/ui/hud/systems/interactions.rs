use bevy::{app::AppExit, prelude::*};

use crate::{
    game::ui::hud::{
        components::{PlayButton, QuitButton},
        styles::{MENU_HOVERED_BUTTON_COLOR, MENU_NORMAL_BUTTON_COLOR, MENU_PRESSED_BUTTON_COLOR},
    },
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    match button_query.get_single_mut() {
        Ok((interaction, mut background_color)) => {
            let (new_color, maybe_new_state) = match *interaction {
                Interaction::Hovered => (MENU_HOVERED_BUTTON_COLOR, None),
                Interaction::Clicked => (MENU_PRESSED_BUTTON_COLOR, Some(AppState::Game)),
                Interaction::None => (MENU_NORMAL_BUTTON_COLOR, None),
            };

            (*background_color) = new_color.into();

            match maybe_new_state {
                Some(state) => app_state_next_state.set(state),
                None => (),
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
