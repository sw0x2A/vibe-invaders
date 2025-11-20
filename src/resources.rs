use bevy::prelude::*;

/// Global game state resource
#[derive(Resource)]
pub struct GameState {
    pub score: u32,
    pub enemy_direction: f32,
    pub enemy_shoot_timer: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            enemy_direction: 1.0,
            enemy_shoot_timer: 0.0,
        }
    }
}
