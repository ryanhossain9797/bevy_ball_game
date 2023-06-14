use bevy::prelude::*;

use crate::component::{enemy::*, star::STAR_SPAWN_TIME};

#[derive(Resource)]
pub struct EnemyTimer(pub Timer);
impl Default for EnemyTimer {
    fn default() -> Self {
        EnemyTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}

#[derive(Resource, Default)]
pub struct HighScore {
    pub high_score: Vec<(String, usize)>,
}

#[derive(Resource)]
pub struct StarTimer(pub Timer);
impl Default for StarTimer {
    fn default() -> Self {
        StarTimer(Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating))
    }
}
