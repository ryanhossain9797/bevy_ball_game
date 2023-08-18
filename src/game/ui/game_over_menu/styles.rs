use bevy::prelude::*;

pub const MENU_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const MENU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const MENU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
pub const MENU_BUTTON_SIZE: Size = Size::new(Val::Px(200.), Val::Px(80.));

pub const MENU_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: MENU_BUTTON_SIZE,
    ..Style::DEFAULT
};

pub const MENU_ITEMS_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    gap: Size::new(Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};

pub const MENU_TITLE_BAR_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.), Val::Px(120.)),
    ..Style::DEFAULT
};

pub const MENU_IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(64.), Val::Px(64.)),
    margin: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 64.,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 64.,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 32.,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}