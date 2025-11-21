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
    pub game_music: Handle<AudioSource>,
}

/// Window dimensions resource
#[derive(Resource)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
        }
    }
}

/// High score entry
#[derive(Clone)]
pub struct ScoreEntry {
    pub score: u32,
    pub is_current: bool,
}

/// High scores resource
#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<ScoreEntry>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self {
            scores: Vec::new(),
        }
    }
}

impl HighScores {
    pub fn add_score(&mut self, score: u32) {
        // Clear current flags
        for entry in &mut self.scores {
            entry.is_current = false;
        }
        
        // Add new score
        self.scores.push(ScoreEntry {
            score,
            is_current: true,
        });
        
        // Sort by score descending
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));
        
        // Keep only top 5
        self.scores.truncate(5);
    }
    
    pub fn get_top_scores(&self) -> &[ScoreEntry] {
        &self.scores
    }
}

/// Game over timer resource for preventing immediate restart
#[derive(Resource)]
pub struct GameOverTimer {
    pub elapsed: f32,
    pub required_delay: f32,
}

impl Default for GameOverTimer {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            required_delay: 1.5, // 1.5 seconds delay
        }
    }
}

impl GameOverTimer {
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }
    
    pub fn can_restart(&self) -> bool {
        self.elapsed >= self.required_delay
    }
}
