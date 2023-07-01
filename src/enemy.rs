use bevy::prelude::*;

use self::{resource::EnemyTimer, system::*};

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
        app.init_resource::<EnemyTimer>()
            .add_startup_system(spawn_initial_enemies)
            .add_system(enemy_movement)
            .add_system(bounce_enemy_movement)
            .add_system(confine_enemy_movement)
            .add_system(enemy_hit_player)
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_occassional_enemies);
    }
}
