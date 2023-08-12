use bevy::prelude::*;
use rand::random;
use std::ops::{Deref, DerefMut};

use crate::game::player::component::Player;
use crate::game::player::HALF_PLAYER_SIZE;
use crate::game::score::resource::Score;
use crate::game::star::component::*;
use crate::game::star::component::*;
use crate::game::star::resource::StarTimer;

use super::{HALF_STAR_SIZE, INITIAL_NUMBER_OF_STARS, STAR_SIZE};

pub fn spawn_initial_stars(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    (0..INITIAL_NUMBER_OF_STARS)
        .into_iter()
        .map(|_| {
            (
                (random::<f32>() * (window.width() - STAR_SIZE)) + (HALF_STAR_SIZE),
                (random::<f32>() * (window.height() - STAR_SIZE)) + (HALF_STAR_SIZE),
            )
        })
        .for_each(|(x, y)| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ));
        })
}

pub fn tick_star_spawn_timer(mut star_timer: ResMut<StarTimer>, time: ResMut<Time>) {
    star_timer.0.tick(time.delta());
}

pub fn spawn_occassional_stars(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut star_timer: ResMut<StarTimer>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    if star_timer.0.finished() {
        let x = (random::<f32>() * (window.width() - STAR_SIZE)) + (HALF_STAR_SIZE);
        let y = (random::<f32>() * (window.height() - STAR_SIZE)) + (HALF_STAR_SIZE);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    star_query
        .iter()
        .for_each(|star| commands.entity(star).despawn())
}
