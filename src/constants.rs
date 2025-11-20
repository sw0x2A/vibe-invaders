// Window and display constants
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

// Player constants
pub const PLAYER_SIZE: f32 = 40.0;
pub const PLAYER_SPEED: f32 = 300.0;

// Bullet constants
pub const BULLET_SIZE: f32 = 5.0;
pub const BULLET_SPEED: f32 = 400.0;

// Enemy constants
pub const ENEMY_SIZE: f32 = 30.0;
pub const ENEMY_SPEED: f32 = 50.0;
pub const ENEMY_ROWS: usize = 5;
pub const ENEMY_COLS: usize = 11;
pub const ENEMY_SPACING: f32 = 60.0;
pub const ENEMY_SHOOT_INTERVAL: f32 = 2.0;

// Starfield constants
pub const STAR_SPAWN_INTERVAL: f32 = 0.05; // Spawn stars frequently
pub const STAR_BASE_SPEED: f32 = 150.0;
pub const STAR_MIN_SIZE: f32 = 1.0;
pub const STAR_MAX_SIZE: f32 = 4.0;
pub const STAR_MIN_BRIGHTNESS: f32 = 0.3;
pub const STAR_MAX_BRIGHTNESS: f32 = 1.0;
pub const STAR_MAX_DISTANCE: f32 = 721.11; // Pre-calculated: sqrt((800/2)^2 + (600/2)^2)

// Explosion constants
pub const EXPLOSION_PARTICLES: usize = 20;
pub const EXPLOSION_LIFETIME: f32 = 0.8;
pub const EXPLOSION_PARTICLE_SPEED: f32 = 200.0;
pub const EXPLOSION_PARTICLE_SIZE: f32 = 3.0;
