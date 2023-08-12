use bevy::prelude::*;

use crate::AppState;

use self::{resource::EnemyTimer, system::*};

use super::SimulationState;

pub mod component;
pub mod resource;
mod system;

pub const NUMBER_OF_ENEMIES: usize = 2;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const HALF_ENEMY_SIZE: f32 = ENEMY_SIZE / 2.0;
pub const ENEMY_SPAWN_TIME: f32 = 4.0;



pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemyTimer>()
        .add_system(spawn_initial_enemies.in_schedule(OnEnter(AppState::Game)))
        .add_systems(
            (
                tick_enemy_spawn_timer,
                spawn_occassional_enemies,
                enemy_hit_player,
                enemy_movement, 
                confine_enemy_movement,
                bounce_enemy_movement,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running)),
        )
        .add_system(
            despawn_enemies
                .in_schedule(OnExit(AppState::Game))
        );
    }
}
