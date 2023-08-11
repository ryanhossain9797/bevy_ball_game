use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod event;
mod game;
mod keyboard_helper;
mod menu;
mod system;

use event::*;
use game::enemy::resource::EnemyTimer;
use game::enemy::*;
use game::player::*;
use game::score::*;
use game::star::resource::StarTimer;
use game::star::*;
use game::GamePlugin;
use menu::MainMenuPlugin;
use rand::*;
use system::*;

fn main() {
    App::new()
        //States
        .add_state::<AppState>()
        //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        //State Transition
        .add_system(transition_to_game_state)
        .add_system(transition_to_menu_state)
        //Systems
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
