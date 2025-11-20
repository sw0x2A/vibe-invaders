use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Initialize the camera and load textures
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    // Load textures
    let textures = GameTextures {
        player: asset_server.load("player.png"),
        enemy1: asset_server.load("enemy1.png"),
        enemy2: asset_server.load("enemy2.png"),
        enemy3: asset_server.load("enemy3.png"),
        bullet: asset_server.load("bullet.png"),
        enemy_bullet: asset_server.load("enemy_bullet.png"),
    };
    
    commands.insert_resource(textures);
}

/// Spawn the player ship and score UI
pub fn spawn_player(mut commands: Commands, textures: Res<GameTextures>) {
    commands.spawn((
        Sprite::from_image(textures.player.clone()),
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 50.0, 0.0),
        Player,
    ));

    // Spawn score text
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Score,
    ));
}

/// Spawn the enemy formation
pub fn spawn_enemies(mut commands: Commands, textures: Res<GameTextures>) {
    let start_x = -(ENEMY_COLS as f32 - 1.0) * ENEMY_SPACING / 2.0;
    let start_y = WINDOW_HEIGHT / 2.0 - 100.0;

    for row in 0..ENEMY_ROWS {
        for col in 0..ENEMY_COLS {
            let x = start_x + col as f32 * ENEMY_SPACING;
            let y = start_y - row as f32 * ENEMY_SPACING;

            // Use different enemy types for different rows
            let enemy_texture = match row {
                0 => textures.enemy3.clone(),
                1..=2 => textures.enemy2.clone(),
                _ => textures.enemy1.clone(),
            };

            commands.spawn((
                Sprite::from_image(enemy_texture),
                Transform::from_xyz(x, y, 0.0),
                Enemy { original_x: x },
                Velocity {
                    x: ENEMY_SPEED,
                    y: 0.0,
                },
            ));
        }
    }
}
