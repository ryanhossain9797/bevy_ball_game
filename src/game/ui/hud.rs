use bevy::prelude::*;

use systems::layout::spawn_hud;

use crate::{game::SimulationState, AppState};

use self::systems::{
    layout::despawn_hud,
    updates::{update_enemy_text, update_score_text},
};

pub mod components;
pub mod styles;
pub mod systems;
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems((update_score_text, update_enemy_text).in_set(OnUpdate(AppState::Game)))
            // OnExit Systems
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
