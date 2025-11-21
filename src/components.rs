use bevy::prelude::*;

/// Player ship component marker
#[derive(Component)]
pub struct Player;

/// Enemy invader component
#[derive(Component)]
pub struct Enemy {
    pub original_x: f32,
}

/// Player bullet component marker
#[derive(Component)]
pub struct Bullet;

/// Enemy bullet component marker
#[derive(Component)]
pub struct EnemyBullet;

/// Velocity component for moving entities
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Score UI component marker
#[derive(Component)]
pub struct Score;

/// Start screen UI component marker
#[derive(Component)]
pub struct StartScreenUI;

/// Game over screen UI component marker
#[derive(Component)]
pub struct GameOverUI;

/// Star component for animated starfield background
#[derive(Component)]
pub struct Star {
    pub distance_from_center: f32,
}

/// Explosion particle component
#[derive(Component)]
pub struct ExplosionParticle {
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// Background music component marker
#[derive(Component)]
pub struct BackgroundMusic;
