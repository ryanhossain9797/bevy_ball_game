use bevy::prelude::*;

use crate::AppState;

use self::system::*;

use super::SimulationState;

pub mod component;
pub mod resource;
mod system;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const HALF_PLAYER_SIZE: f32 = PLAYER_SIZE / 2.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    MovementSystemSet,
    ConfinementSystemSet,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PlayerSystemSet::ConfinementSystemSet.after(PlayerSystemSet::MovementSystemSet),
        )
        .add_startup_system(spawn_player)
        .add_system(
            player_movement
                .in_set(PlayerSystemSet::MovementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        )
        .add_system(
            confine_player_movement
                .in_set(PlayerSystemSet::ConfinementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        )
        .add_system(
            player_pickup_star
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
