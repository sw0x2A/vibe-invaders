use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Setup start screen
pub fn setup_start_screen(mut commands: Commands) {
    // Title text
    commands.spawn((
        Text::new("VIBE INVADERS"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.5)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(150.0),
            left: Val::Px(WINDOW_WIDTH / 2.0 - 200.0),
            ..default()
        },
        StartScreenUI,
    ));

    // Instructions text
    commands.spawn((
        Text::new("Press SPACE to Start"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(WINDOW_WIDTH / 2.0 - 150.0),
            ..default()
        },
        StartScreenUI,
    ));

    // Controls text
    commands.spawn((
        Text::new("Controls:\nArrow Keys or A/D - Move\nSPACE - Shoot"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(400.0),
            left: Val::Px(WINDOW_WIDTH / 2.0 - 150.0),
            ..default()
        },
        StartScreenUI,
    ));
}

/// Handle start screen input
pub fn start_screen_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GamePhase::Playing);
    }
}

/// Cleanup start screen
pub fn cleanup_start_screen(mut commands: Commands, query: Query<Entity, With<StartScreenUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Setup game over screen
pub fn setup_game_over_screen(mut commands: Commands, game_state: Res<GameState>) {
    // Game over text
    commands.spawn((
        Text::new("GAME OVER"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(150.0),
            left: Val::Px(WINDOW_WIDTH / 2.0 - 180.0),
            ..default()
        },
        GameOverUI,
    ));

    // Final score text
    commands.spawn((
        Text::new(format!("Final Score: {}", game_state.score)),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(250.0),
            left: Val::Px(WINDOW_WIDTH / 2.0 - 150.0),
            ..default()
        },
        GameOverUI,
    ));

    // Restart text
    commands.spawn((
        Text::new("Press SPACE to Restart"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(WINDOW_WIDTH / 2.0 - 180.0),
            ..default()
        },
        GameOverUI,
    ));
}

/// Handle game over screen input
pub fn game_over_screen_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GamePhase::Playing);
    }
}

/// Cleanup game over screen
pub fn cleanup_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Cleanup game entities (player, enemies, bullets)
pub fn cleanup_game_entities(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
    enemy_bullet_query: Query<Entity, With<EnemyBullet>>,
    score_query: Query<Entity, With<Score>>,
    mut game_state: ResMut<GameState>,
) {
    // Despawn player
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn enemies
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn bullets
    for entity in bullet_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn enemy bullets
    for entity in enemy_bullet_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn score UI
    for entity in score_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Reset game state
    game_state.reset();
}
