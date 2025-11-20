use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Handle player movement
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        let new_x = transform.translation.x + direction * PLAYER_SPEED * time.delta_secs();
        let half_player = PLAYER_SIZE / 2.0;
        transform.translation.x = new_x.clamp(
            -WINDOW_WIDTH / 2.0 + half_player,
            WINDOW_WIDTH / 2.0 - half_player,
        );
    }
}

/// Handle player shooting
pub fn player_shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    textures: Res<GameTextures>,
) {
    if keyboard_input.just_pressed(KeyCode::Space)
        && let Ok(transform) = query.single()
    {
        commands.spawn((
            Sprite::from_image(textures.bullet.clone()),
            Transform::from_xyz(
                transform.translation.x,
                transform.translation.y + PLAYER_SIZE / 2.0,
                0.0,
            ),
            Bullet,
            Velocity {
                x: 0.0,
                y: BULLET_SPEED,
            },
        ));
    }
}
