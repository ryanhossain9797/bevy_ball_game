pub mod components;
pub mod styles;
pub mod systems;

use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::AppState;

use self::systems::{
    interactions::{
        interact_with_main_menu_button, interact_with_quit_button, interact_with_restart_button,
    },
    layout::{despawn_game_over_menu, spawn_game_over_menu},
    updates::update_final_score_text,
};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_systems(
                (
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    interact_with_restart_button,
                    update_final_score_text,
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
