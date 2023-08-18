pub mod component;
pub mod resource;
mod system;

use crate::AppState;

use self::{resource::*, system::*};
use bevy::prelude::*;

use super::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(
                Update,
                (display_score, update_high_scores, on_high_scores_updated)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
