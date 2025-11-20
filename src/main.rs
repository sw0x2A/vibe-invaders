mod components;
mod constants;
mod resources;
mod systems;

use bevy::prelude::*;

use constants::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Vibe Invaders".to_string(),
                    resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: "resources".to_string(),
                ..default()
            })
        )
        .init_state::<GamePhase>()
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        // Start screen systems
        .add_systems(OnEnter(GamePhase::StartScreen), setup_start_screen)
        .add_systems(Update, start_screen_input.run_if(in_state(GamePhase::StartScreen)))
        .add_systems(OnExit(GamePhase::StartScreen), cleanup_start_screen)
        // Game playing systems
        .add_systems(OnEnter(GamePhase::Playing), (spawn_player, spawn_enemies))
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
                check_all_enemies_destroyed,
                update_score_display,
                cleanup_offscreen_bullets,
            ).run_if(in_state(GamePhase::Playing)),
        )
        .add_systems(OnExit(GamePhase::Playing), cleanup_game_entities)
        // Game over systems
        .add_systems(OnEnter(GamePhase::GameOver), setup_game_over_screen)
        .add_systems(Update, game_over_screen_input.run_if(in_state(GamePhase::GameOver)))
        .add_systems(OnExit(GamePhase::GameOver), cleanup_game_over_screen)
        .run();
}
