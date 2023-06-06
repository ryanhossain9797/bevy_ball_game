use crate::{
    entity::{enemy::*, player::*},
    keyboard_helper::*,
    *,
};
use bevy::prelude::*;

pub fn spawn_initial_enemies(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    (0..NUMBER_OF_ENEMIES)
        .into_iter()
        .map(|_| {
            (
                (random::<f32>() * (window.width() - ENEMY_SIZE)) + (HALF_ENEMY_SIZE),
                (random::<f32>() * (window.height() - ENEMY_SIZE)) + (HALF_ENEMY_SIZE),
                Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            )
        })
        .for_each(|(x, y, direction)| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy { direction },
            ));
        })
}

#[derive(Resource)]
pub struct EnemyTimer(pub Timer);
impl Default for EnemyTimer {
    fn default() -> Self {
        EnemyTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_timer: ResMut<EnemyTimer>, time: Res<Time>) {
    enemy_timer.0.tick(time.delta());
}

pub fn spawn_occassional_enemies(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut enemy_timer: ResMut<EnemyTimer>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    if enemy_timer.0.finished() {
        let x = (random::<f32>() * (window.width() - ENEMY_SIZE)) + (HALF_ENEMY_SIZE);
        let y = (random::<f32>() * (window.height() - ENEMY_SIZE)) + (HALF_ENEMY_SIZE);
        let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy { direction },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    enemy_query.for_each_mut(|(mut transform, enemy)| {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0).normalize_or_zero();

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    });
}

pub fn bounce_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");

    enemy_query.for_each_mut(|(transform, mut enemy)| {
        let translation = &transform.translation;
        let invert_x = {
            let x_min = 0.0 + HALF_ENEMY_SIZE;
            let x_max = window.width() - HALF_ENEMY_SIZE;

            (translation.x < x_min && enemy.direction.x < 0.0)
                || (translation.x > x_max && enemy.direction.x >= 0.0)
        };
        let invert_y = {
            let y_min = 0.0 + HALF_ENEMY_SIZE;
            let y_max = window.height() - HALF_ENEMY_SIZE;

            (translation.y < y_min && enemy.direction.y < 0.0)
                || (translation.y > y_max && enemy.direction.y >= 0.0)
        };
        if invert_x || invert_y {
            let bounce_sound_1 = asset_server.load("audio/pluck_001.ogg");
            audio.play(bounce_sound_1);

            let mut direction = enemy.direction;

            if (invert_x) {
                direction.x = direction.x * -1.0
            }
            if invert_y {
                direction.y = direction.y * -1.0
            }

            enemy.direction = direction;
        }
    });
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary windows not found");
    enemy_query.for_each_mut(|mut transform| {
        let mut translation = transform.translation;
        {
            let x_min = 0.0 + HALF_ENEMY_SIZE;
            let x_max = window.width() - HALF_ENEMY_SIZE;
            if translation.x < x_min {
                translation.x = x_min;
            } else if translation.x > x_max {
                translation.x = x_max;
            }
        }
        {
            let y_min = 0.0 + HALF_ENEMY_SIZE;
            let y_max = window.height() - HALF_ENEMY_SIZE;

            if translation.y < y_min {
                translation.y = y_min;
            } else if translation.y > y_max {
                translation.y = y_max;
            }
        }
        transform.translation = translation;
    });
}

pub fn enemy_hit_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        enemy_query.for_each(|enemy_transform| {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < HALF_ENEMY_SIZE + HALF_PLAYER_SIZE {
                println!("Game over!");
                let explosion_sound = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(explosion_sound);
                commands.entity(player_entity).despawn();
            }
        });
    }
}
