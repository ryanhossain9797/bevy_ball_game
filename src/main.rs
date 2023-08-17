use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod event;
mod game;
mod keyboard_helper;
mod main_menu;
mod systems;

use game::ui::GameUiPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        //States
        .add_state::<AppState>()
        //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameUiPlugin)
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
