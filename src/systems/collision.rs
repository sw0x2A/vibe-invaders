use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::systems::explosion::spawn_explosion;

/// Check for bullet-enemy collisions
pub fn check_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut game_state: ResMut<GameState>,
    audio: Res<GameAudio>,
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

                // Spawn explosion with orange/yellow color
                spawn_explosion(
                    &mut commands,
                    enemy_transform.translation,
                    Color::srgb(1.0, 0.6, 0.0),
                );

                // Play enemy destroyed sound
                commands.spawn((
                    AudioPlayer::new(audio.enemy_destroyed.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}

/// Check for enemy bullet-player collisions
pub fn check_bullet_player_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    audio: Res<GameAudio>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(player_transform.translation);

            if distance < (BULLET_SIZE + PLAYER_SIZE) / 2.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(player_entity).despawn();

                // Spawn explosion with red/orange color
                spawn_explosion(
                    &mut commands,
                    player_transform.translation,
                    Color::srgb(1.0, 0.3, 0.0),
                );

                // Play player destroyed sound
                commands.spawn((
                    AudioPlayer::new(audio.player_destroyed.clone()),
                    PlaybackSettings::DESPAWN,
                ));

                next_state.set(GamePhase::GameOver);
                break;
            }
        }
    }
}
