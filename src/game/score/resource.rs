use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}

#[derive(Resource, Default, Debug)]
pub struct HighScore {
    pub high_score: Vec<(String, usize)>,
}
