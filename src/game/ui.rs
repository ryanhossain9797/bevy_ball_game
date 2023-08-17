use bevy::prelude::{App, Plugin};

use self::{game_over_menu::GameOverMenuPlugin, hud::HudPlugin, pause_menu::PauseMenuPlugin};

pub mod game_over_menu;
pub mod hud;
pub mod pause_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverMenuPlugin);
    }
}
