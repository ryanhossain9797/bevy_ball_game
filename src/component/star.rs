use crate::*;
use bevy::prelude::*;

pub const INITIAL_NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const HALF_STAR_SIZE: f32 = STAR_SIZE / 2.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Component)]
pub struct Star {}
