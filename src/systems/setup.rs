use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

/// Initialize the camera
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Spawn the player ship and score UI
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..default()
        },
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
pub fn spawn_enemies(mut commands: Commands) {
    let start_x = -(ENEMY_COLS as f32 - 1.0) * ENEMY_SPACING / 2.0;
    let start_y = WINDOW_HEIGHT / 2.0 - 100.0;

    for row in 0..ENEMY_ROWS {
        for col in 0..ENEMY_COLS {
            let x = start_x + col as f32 * ENEMY_SPACING;
            let y = start_y - row as f32 * ENEMY_SPACING;

            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
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
