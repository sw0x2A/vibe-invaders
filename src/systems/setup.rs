use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

    // Load audio
    let audio = GameAudio {
        player_shoot: asset_server.load("sounds/player_shoot.wav"),
        enemy_shoot: asset_server.load("sounds/enemy_shoot.wav"),
        enemy_destroyed: asset_server.load("sounds/enemy_destroyed.wav"),
        player_destroyed: asset_server.load("sounds/player_destroyed.wav"),
        game_music: asset_server.load("sounds/game_music.wav"),
    };

    commands.insert_resource(audio);
}

/// Initialize window dimensions from actual window size
pub fn initialize_window_dimensions(
    mut window_dims: ResMut<WindowDimensions>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.single() {
        window_dims.width = window.width();
        window_dims.height = window.height();
    }
}

/// Spawn the player ship and score UI
pub fn spawn_player(
    mut commands: Commands,
    textures: Res<GameTextures>,
    audio: Res<GameAudio>,
    window_dims: Res<WindowDimensions>,
) {
    commands.spawn((
        Sprite {
            image: textures.player.clone(),
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, -window_dims.height / 2.0 + 50.0, 0.0),
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
        TextLayout::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Score,
    ));

    // Start background music
    commands.spawn((
        AudioPlayer::new(audio.game_music.clone()),
        PlaybackSettings::LOOP,
        BackgroundMusic,
    ));
}

/// Spawn the enemy formation
pub fn spawn_enemies(
    mut commands: Commands,
    textures: Res<GameTextures>,
    window_dims: Res<WindowDimensions>,
) {
    let start_x = -(ENEMY_COLS as f32 - 1.0) * ENEMY_SPACING / 2.0;
    let start_y = window_dims.height / 2.0 - 100.0;

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
                Sprite {
                    image: enemy_texture,
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
