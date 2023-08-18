use bevy::prelude::*;

use crate::AppState;

use self::{resource::StarTimer, system::*};

use super::SimulationState;

pub mod component;
pub mod resource;
mod system;

pub const INITIAL_NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const HALF_STAR_SIZE: f32 = STAR_SIZE / 2.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_initial_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_occassional_stars)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
