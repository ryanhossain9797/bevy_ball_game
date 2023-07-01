use bevy::prelude::*;
use rand::random;
use std::ops::{Deref, DerefMut};

use crate::event::GameOver;

use super::resource::HighScore;

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScore>,
) {
    if !game_over_event_reader.is_empty() {
        game_over_event_reader.iter().for_each(|event| {
            high_scores
                .high_score
                .push(("Player".to_string(), event.score))
        });
    }
}

pub fn on_high_scores_updated(mut high_scores: Res<HighScore>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores)
    }
}
