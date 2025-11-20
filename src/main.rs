use bevy::prelude::*;
use rand::Rng;

// Constants
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SIZE: f32 = 40.0;
const PLAYER_SPEED: f32 = 300.0;
const BULLET_SIZE: f32 = 5.0;
const BULLET_SPEED: f32 = 400.0;
const ENEMY_SIZE: f32 = 30.0;
const ENEMY_SPEED: f32 = 50.0;
const ENEMY_ROWS: usize = 5;
const ENEMY_COLS: usize = 11;
const ENEMY_SPACING: f32 = 60.0;
const ENEMY_SHOOT_INTERVAL: f32 = 2.0;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    original_x: f32,
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct EnemyBullet;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Score;

// Resources
#[derive(Resource)]
struct GameState {
    score: u32,
    enemy_direction: f32,
    enemy_shoot_timer: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            enemy_direction: 1.0,
            enemy_shoot_timer: 0.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vibe Invaders".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(Startup, (setup, spawn_player, spawn_enemies))
        .add_systems(
            Update,
            (
                player_movement,
                player_shoot,
                move_bullets,
                move_enemy_bullets,
                move_enemies,
                enemy_shoot,
                check_bullet_enemy_collision,
                check_bullet_player_collision,
                check_enemy_reached_bottom,
                update_score_display,
                cleanup_offscreen_bullets,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 50.0, 0.0),
            ..default()
        },
        Player,
    ));

    // Spawn score text
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        Score,
    ));
}

fn spawn_enemies(mut commands: Commands) {
    let start_x = -(ENEMY_COLS as f32 - 1.0) * ENEMY_SPACING / 2.0;
    let start_y = WINDOW_HEIGHT / 2.0 - 100.0;

    for row in 0..ENEMY_ROWS {
        for col in 0..ENEMY_COLS {
            let x = start_x + col as f32 * ENEMY_SPACING;
            let y = start_y - row as f32 * ENEMY_SPACING;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Enemy { original_x: x },
                Velocity {
                    x: ENEMY_SPEED,
                    y: 0.0,
                },
            ));
        }
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        let new_x = transform.translation.x + direction * PLAYER_SPEED * time.delta_seconds();
        let half_player = PLAYER_SIZE / 2.0;
        transform.translation.x = new_x.clamp(
            -WINDOW_WIDTH / 2.0 + half_player,
            WINDOW_WIDTH / 2.0 - half_player,
        );
    }
}

fn player_shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(transform) = query.get_single() {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE * 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y + PLAYER_SIZE / 2.0,
                        0.0,
                    ),
                    ..default()
                },
                Bullet,
                Velocity {
                    x: 0.0,
                    y: BULLET_SPEED,
                },
            ));
        }
    }
}

fn move_bullets(mut query: Query<(&mut Transform, &Velocity), With<Bullet>>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn move_enemy_bullets(
    mut query: Query<(&mut Transform, &Velocity), With<EnemyBullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn move_enemies(
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
        transform.translation.x += velocity.x * time.delta_seconds();

        if should_move_down {
            transform.translation.y -= 20.0;
        }
    }
}

fn enemy_shoot(
    mut commands: Commands,
    query: Query<&Transform, With<Enemy>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    game_state.enemy_shoot_timer += time.delta_seconds();

    if game_state.enemy_shoot_timer >= ENEMY_SHOOT_INTERVAL {
        game_state.enemy_shoot_timer = 0.0;

        // Pick a random enemy to shoot
        let enemies: Vec<&Transform> = query.iter().collect();
        if !enemies.is_empty() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..enemies.len());
            let enemy_transform = enemies[index];

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.5, 0.0),
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE * 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        enemy_transform.translation.x,
                        enemy_transform.translation.y - ENEMY_SIZE / 2.0,
                        0.0,
                    ),
                    ..default()
                },
                EnemyBullet,
                Velocity {
                    x: 0.0,
                    y: -BULLET_SPEED,
                },
            ));
        }
    }
}

fn check_bullet_enemy_collision(
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

fn check_bullet_player_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if let Ok((_player_entity, player_transform)) = player_query.get_single() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(player_transform.translation);

            if distance < (BULLET_SIZE + PLAYER_SIZE) / 2.0 {
                commands.entity(bullet_entity).despawn();
                // In a full game, you'd handle game over here
                println!("Player hit! Game Over!");
            }
        }
    }
}

fn check_enemy_reached_bottom(query: Query<&Transform, With<Enemy>>) {
    for transform in query.iter() {
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 + 50.0 {
            println!("Enemies reached the bottom! Game Over!");
            // In a full game, you'd handle game over here
        }
    }
}

fn update_score_display(game_state: Res<GameState>, mut query: Query<&mut Text, With<Score>>) {
    if game_state.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Score: {}", game_state.score);
        }
    }
}

fn cleanup_offscreen_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    for (entity, transform) in bullet_query.iter() {
        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 10.0 {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform) in enemy_bullet_query.iter() {
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 - 10.0 {
            commands.entity(entity).despawn();
        }
    }
}
