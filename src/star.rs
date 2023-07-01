use bevy::prelude::{App, Plugin};

use self::{resource::StarTimer, system::*};

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
            .add_startup_system(spawn_initial_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_occassional_stars);
    }
}
