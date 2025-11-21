use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Setup start screen
pub fn setup_start_screen(mut commands: Commands, window_dims: Res<WindowDimensions>) {
    // Title text
    commands.spawn((
        Text::new("VIBE INVADERS"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.5)),
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(150.0),
            left: Val::Px(window_dims.width / 2.0 - 200.0),
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
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(window_dims.width / 2.0 - 150.0),
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
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(400.0),
            left: Val::Px(window_dims.width / 2.0 - 150.0),
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
pub fn setup_game_over_screen(
    mut commands: Commands, 
    game_state: Res<GameState>, 
    mut high_scores: ResMut<HighScores>,
    mut game_over_timer: ResMut<GameOverTimer>,
    window_dims: Res<WindowDimensions>,
) {
    // Reset timer
    game_over_timer.reset();
    
    // Add current score to high scores
    high_scores.add_score(game_state.score);
    
    let (title, title_color) = if game_state.victory {
        ("VICTORY!", Color::srgb(0.0, 1.0, 0.0))
    } else {
        ("GAME OVER", Color::srgb(1.0, 0.0, 0.0))
    };

    // Game over or victory text
    commands.spawn((
        Text::new(title),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(title_color),
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(window_dims.width / 2.0 - 180.0),
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
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(130.0),
            left: Val::Px(window_dims.width / 2.0 - 150.0),
            ..default()
        },
        GameOverUI,
    ));

    // High scores title
    commands.spawn((
        Text::new("HIGH SCORES"),
        TextFont {
            font_size: 35.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.84, 0.0)),
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(window_dims.width / 2.0 - 100.0),
            ..default()
        },
        GameOverUI,
    ));

    // Display top 5 scores
    let top_scores = high_scores.get_top_scores();
    for (i, score_entry) in top_scores.iter().enumerate() {
        let y_pos = 250.0 + i as f32 * 40.0;
        let score_text = format!("{}. {}", i + 1, score_entry.score);
        let color = if score_entry.is_current {
            Color::srgb(0.0, 1.0, 0.5) // Highlight current score
        } else {
            Color::WHITE
        };
        
        commands.spawn((
            Text::new(score_text),
            TextFont {
                font_size: 28.0,
                ..default()
            },
            TextColor(color),
            TextLayout::default(),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(y_pos),
                left: Val::Px(window_dims.width / 2.0 - 60.0),
                ..default()
            },
            GameOverUI,
        ));
    }

    // Restart text
    commands.spawn((
        Text::new("Press SPACE to Restart"),
        TextFont {
            font_size: 25.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(window_dims.height - 80.0),
            left: Val::Px(window_dims.width / 2.0 - 150.0),
            ..default()
        },
        GameOverUI,
    ));
}

/// Handle game over screen input
pub fn game_over_screen_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut game_over_timer: ResMut<GameOverTimer>,
    time: Res<Time>,
) {
    // Update timer
    game_over_timer.elapsed += time.delta_secs();
    
    // Only allow restart after delay
    if keyboard_input.just_pressed(KeyCode::Space) && game_over_timer.can_restart() {
        next_state.set(GamePhase::Playing);
    }
}

/// Cleanup game over screen
pub fn cleanup_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUI>>,
    mut game_state: ResMut<GameState>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Reset game state for next game
    game_state.reset();
}

/// Cleanup game entities (player, enemies, bullets)
pub fn cleanup_game_entities(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
    enemy_bullet_query: Query<Entity, With<EnemyBullet>>,
    score_query: Query<Entity, With<Score>>,
    music_query: Query<Entity, With<BackgroundMusic>>,
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
    
    // Despawn background music
    for entity in music_query.iter() {
        commands.entity(entity).despawn();
    }
}
