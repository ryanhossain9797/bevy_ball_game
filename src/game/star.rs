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
            .add_system(
                spawn_initial_stars
                    .in_schedule(OnEnter(AppState::Game))
            )
            .add_systems(
                (
                    tick_star_spawn_timer, 
                    spawn_occassional_stars
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                despawn_stars
                    .in_schedule(OnExit(AppState::Game))
            );
    }
}
