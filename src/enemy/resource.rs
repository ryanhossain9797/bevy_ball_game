use bevy::prelude::*;

use super::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct EnemyTimer(pub Timer);
impl Default for EnemyTimer {
    fn default() -> Self {
        EnemyTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}
