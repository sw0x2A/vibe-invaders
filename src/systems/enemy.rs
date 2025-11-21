use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Move enemies in formation
pub fn move_enemies(
    mut query: Query<(&mut Transform, &mut Velocity, &Enemy)>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    let mut should_move_down = false;
    let mut reverse_direction = false;

    // Check if any enemy hit the edge
    for (transform, _, enemy) in query.iter() {
        let offset = transform.translation.x - enemy.original_x;
        if offset.abs() > 100.0 {
            reverse_direction = true;
            should_move_down = true;
            break;
        }
    }

    if reverse_direction {
        game_state.enemy_direction *= -1.0;
    }

    // Move enemies
    for (mut transform, mut velocity, _) in query.iter_mut() {
        velocity.x = ENEMY_SPEED * game_state.enemy_direction;
        transform.translation.x += velocity.x * time.delta_secs();

        if should_move_down {
            transform.translation.y -= 20.0;
        }
    }
}

/// Handle enemy shooting
pub fn enemy_shoot(
    mut commands: Commands,
    query: Query<&Transform, With<Enemy>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    textures: Res<GameTextures>,
    audio: Res<GameAudio>,
) {
    game_state.enemy_shoot_timer += time.delta_secs();

    if game_state.enemy_shoot_timer >= ENEMY_SHOOT_INTERVAL {
        game_state.enemy_shoot_timer = 0.0;

        // Pick a random enemy to shoot
        let enemies: Vec<&Transform> = query.iter().collect();
        if !enemies.is_empty() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..enemies.len());
            let enemy_transform = enemies[index];

            commands.spawn((
                Sprite {
                    image: textures.enemy_bullet.clone(),
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE * 2.0)),
                    ..default()
                },
                Transform::from_xyz(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y - ENEMY_SIZE / 2.0,
                    0.0,
                ),
                EnemyBullet,
                Velocity {
                    x: 0.0,
                    y: -BULLET_SPEED,
                },
            ));
            
            // Play enemy shoot sound
            commands.spawn((
                AudioPlayer::new(audio.enemy_shoot.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

/// Check if enemies reached the bottom
pub fn check_enemy_reached_bottom(
    query: Query<&Transform, With<Enemy>>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    for transform in query.iter() {
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 + 50.0 {
            next_state.set(GamePhase::GameOver);
            break;
        }
    }
}
