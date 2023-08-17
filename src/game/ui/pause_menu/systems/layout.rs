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
    commands
        .spawn((
            PauseMenu {},
            NodeBundle {
                style: MENU_ITEMS_STYLE,
                background_color: Color::rgba(0., 0., 0., 0.2).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            //===Title===
            parent
                .spawn(NodeBundle {
                    style: MENU_TITLE_BAR_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    let image = ImageBundle {
                        style: MENU_IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    };
                    parent.spawn(image.clone());
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Paused",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(image);
                });
            //===Play Button===
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
                        text: Text {
                            sections: vec![TextSection::new(
                                "Resume",
                                get_button_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //===Quit Button===
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
                        text: Text {
                            sections: vec![TextSection::new(
                                "Main Menu",
                                get_button_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id()
}
