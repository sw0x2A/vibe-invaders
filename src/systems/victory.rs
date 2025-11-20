use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Check if all enemies are destroyed (victory condition)
pub fn check_all_enemies_destroyed(
    enemy_query: Query<Entity, With<Enemy>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut game_state: ResMut<GameState>,
) {
    if enemy_query.is_empty() {
        game_state.victory = true;
        next_state.set(GamePhase::GameOver);
    }
}
