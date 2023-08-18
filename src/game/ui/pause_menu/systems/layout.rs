use bevy::prelude::*;

use crate::game::ui::pause_menu::{
    components::{MainMenuButton, PauseMenu, ResumeButton},
    styles::*,
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _ = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(mut commands: Commands, main_menu_query: Query<Entity, With<PauseMenu>>) {
    let main_menu = main_menu_query.get_single().expect("Main menu not found");

    commands.entity(main_menu).despawn_recursive();
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: PAUSE_MENU_CONTAINER_STYLE,
                    background_color: PAUSE_MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Pause Menu",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Resume Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: MENU_BUTTON_STYLE,
                                background_color: MENU_NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: MENU_BUTTON_STYLE,
                                background_color: MENU_NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    pause_menu_entity
}
