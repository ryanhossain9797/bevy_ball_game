use bevy::prelude::*;

use crate::game::ui::game_over_menu::{
    components::{GameOverMenu, MainMenuButton, QuitButton, RestartButton},
    styles::*,
};

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _ = build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    let game_over_menu = game_over_menu_query
        .get_single()
        .expect("Main menu not found");

    commands.entity(game_over_menu).despawn_recursive();
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            GameOverMenu {},
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
                                "Game Over",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(image);
                });
            //===Restart Button===
            parent
                .spawn((
                    ButtonBundle {
                        style: MENU_BUTTON_STYLE,
                        background_color: MENU_NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    RestartButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Restart",
                                get_button_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            //===Main Menu Button===
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
            //===Quit Button===
            parent
                .spawn((
                    ButtonBundle {
                        style: MENU_BUTTON_STYLE,
                        background_color: MENU_NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
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
