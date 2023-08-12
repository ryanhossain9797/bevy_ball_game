use bevy::prelude::*;

pub const MENU_NORMAL_BUTTON_COLOR: Color = Color::DARK_GRAY;
pub const MENU_BUTTON_SIZE: Size = Size::new(Val::Px(200.), Val::Px(80.));

pub const MENU_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: MENU_BUTTON_SIZE,
    ..Style::DEFAULT
};
