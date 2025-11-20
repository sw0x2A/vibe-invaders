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
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, (spawn_player, spawn_enemies).after(setup))
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
