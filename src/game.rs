use self::{
    enemy::EnemyPlugin,
    player::PlayerPlugin,
    score::ScorePlugin,
    star::StarPlugin,
    system::{pause_simulation, resume_simulation, toggle_simulation},
};
use crate::{event::GameOver, AppState};
use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod system;
pub mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins((EnemyPlugin, PlayerPlugin, StarPlugin, ScorePlugin))
            .add_systems(OnExit(AppState::Game), resume_simulation)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
