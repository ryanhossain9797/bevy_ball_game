use bevy::prelude::{App, Plugin};

pub mod components;
pub mod styles;
pub mod systems;
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {}
}
