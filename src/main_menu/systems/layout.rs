use bevy::prelude::*;

use crate::main_menu::{self, components::MainMenu};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    let main_menu = main_menu_query.get_single().expect("Main menu not found");

    commands.entity(main_menu).despawn_recursive();
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            MainMenu {},
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
        ))
        .id()
}
