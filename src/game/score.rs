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
        app
            .init_resource::<HighScore>()
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (display_score, update_high_scores, on_high_scores_updated)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
