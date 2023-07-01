use bevy::prelude::*;

use super::STAR_SPAWN_TIME;

#[derive(Resource)]
pub struct StarTimer(pub Timer);
impl Default for StarTimer {
    fn default() -> Self {
        StarTimer(Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating))
    }
}
