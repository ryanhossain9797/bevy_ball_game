use bevy::prelude::*;
use rand::random;
use std::ops::{Deref, DerefMut};

use crate::component::player::*;
use crate::component::star::HALF_STAR_SIZE;
use crate::component::star::INITIAL_NUMBER_OF_STARS;
use crate::component::star::STAR_SIZE;
use crate::component::star::STAR_SPAWN_TIME;
use crate::component::star::*;
use crate::resource::Score;
use crate::resource::StarTimer;
use crate::system::player::*;

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

pub fn player_pickup_star(
    mut commands: Commands,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut player_query: Query<&Transform, With<Player>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    match player_query.get_single_mut() {
        Ok(player_transform) => {
            star_query.for_each(|(star_entity, star_transform)| {
                let distance = player_transform
                    .translation
                    .distance(star_transform.translation);

                if distance < HALF_STAR_SIZE + HALF_PLAYER_SIZE {
                    let pickup_sound = asset_server.load("audio/laserLarge_000.ogg");
                    audio.play(pickup_sound);
                    commands.entity(star_entity).despawn();
                    score.value += 1;
                }
            });
        }
        _ => (),
    }
}
