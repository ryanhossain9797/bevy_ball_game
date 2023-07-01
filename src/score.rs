use bevy::prelude::{App, Plugin};

use self::{resource::*, system::*};

pub mod component;
pub mod resource;
mod system;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_system(update_high_scores)
            .add_system(on_high_scores_updated);
    }
}
