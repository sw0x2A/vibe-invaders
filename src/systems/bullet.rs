use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Move player bullets
pub fn move_bullets(mut query: Query<(&mut Transform, &Velocity), With<Bullet>>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

/// Move enemy bullets
pub fn move_enemy_bullets(
    mut query: Query<(&mut Transform, &Velocity), With<EnemyBullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

/// Clean up bullets that went off-screen
pub fn cleanup_offscreen_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    window_dims: Res<WindowDimensions>,
) {
    for (entity, transform) in bullet_query.iter() {
        if transform.translation.y > window_dims.height / 2.0 + 10.0 {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform) in enemy_bullet_query.iter() {
        if transform.translation.y < -window_dims.height / 2.0 - 10.0 {
            commands.entity(entity).despawn();
        }
    }
}
