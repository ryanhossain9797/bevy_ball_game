use crate::*;
use bevy::prelude::*;

pub const NUMBER_OF_ENEMIES: usize = 2;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const HALF_ENEMY_SIZE: f32 = ENEMY_SIZE / 2.0;
pub const ENEMY_SPAWN_TIME: f32 = 4.0;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
