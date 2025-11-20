use bevy::prelude::*;

/// Game phase states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GamePhase {
    #[default]
    StartScreen,
    Playing,
    GameOver,
}

/// Global game state resource
#[derive(Resource)]
pub struct GameState {
    pub score: u32,
    pub enemy_direction: f32,
    pub enemy_shoot_timer: f32,
    pub star_spawn_timer: f32,
    pub victory: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            enemy_direction: 1.0,
            enemy_shoot_timer: 0.0,
            star_spawn_timer: 0.0,
            victory: false,
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        self.score = 0;
        self.enemy_direction = 1.0;
        self.enemy_shoot_timer = 0.0;
        self.star_spawn_timer = 0.0;
        self.victory = false;
    }
}

/// Texture handles resource
#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub enemy1: Handle<Image>,
    pub enemy2: Handle<Image>,
    pub enemy3: Handle<Image>,
    pub bullet: Handle<Image>,
    pub enemy_bullet: Handle<Image>,
}

/// Audio handles resource
#[derive(Resource)]
pub struct GameAudio {
    pub player_shoot: Handle<AudioSource>,
    pub enemy_shoot: Handle<AudioSource>,
    pub enemy_destroyed: Handle<AudioSource>,
    pub player_destroyed: Handle<AudioSource>,
}
