use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Check for bullet-enemy collisions
pub fn check_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut game_state: ResMut<GameState>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < (BULLET_SIZE + ENEMY_SIZE) / 2.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                game_state.score += 10;
            }
        }
    }
}

/// Check for enemy bullet-player collisions
pub fn check_bullet_player_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    player_query: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(player_transform.translation);

            if distance < (BULLET_SIZE + PLAYER_SIZE) / 2.0 {
                commands.entity(bullet_entity).despawn();
                next_state.set(GamePhase::GameOver);
                break;
            }
        }
    }
}
