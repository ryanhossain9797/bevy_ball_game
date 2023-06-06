use super::star::Score;
use bevy::prelude::{DetectChanges, Res};

pub fn display_score(score: Res<Score>) {
    match score.is_changed() {
        true => println!("Score is {}", score.value),
        false => (),
    }
}
