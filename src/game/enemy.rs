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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EnemySystemSet {
    MovementSystemSet,
    ConfinementSystemSet,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            EnemySystemSet::MovementSystemSet.after(EnemySystemSet::ConfinementSystemSet),
        )
        .init_resource::<EnemyTimer>()
        .add_startup_system(spawn_initial_enemies)
        .add_systems(
            (
                tick_enemy_spawn_timer,
                spawn_occassional_enemies,
                enemy_hit_player,
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        )
        .add_systems(
            (enemy_movement, bounce_enemy_movement)
                .in_set(EnemySystemSet::MovementSystemSet)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        )
        .add_system(
            (confine_enemy_movement)
                .in_set(EnemySystemSet::ConfinementSystemSet)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        );
    }
}
