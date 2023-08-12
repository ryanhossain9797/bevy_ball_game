use bevy::prelude::*;

use crate::main_menu::{
    self,
    components::{MainMenu, PlayButton, QuitButton},
    styles::*,
};

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
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    gap: Size::new(Val::Px(8.), Val::Px(8.)),
                    ..default()
                },
                background_color: Color::rgba(0., 0., 0., 0.2).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            //===Title===
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Bevy Ball Game",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::WHITE,
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            //===Play Button===
            parent
                .spawn((
                    ButtonBundle {
                        style: MENU_BUTTON_STYLE,
                        background_color: MENU_NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                },
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
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                },
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
