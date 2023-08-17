use bevy::prelude::*;

use crate::{game::SimulationState, AppState};

use self::systems::{
    interactions::{interact_with_main_menu_button, interact_with_resume_button},
    layout::{despawn_pause_menu, spawn_pause_menu},
};

pub mod components;
pub mod styles;
pub mod systems;
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            .add_systems(
                (interact_with_resume_button, interact_with_main_menu_button)
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}
