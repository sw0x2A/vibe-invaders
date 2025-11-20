use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Update the score display
pub fn update_score_display(game_state: Res<GameState>, mut query: Query<&mut Text, With<Score>>) {
    if game_state.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("Score: {}", game_state.score);
        }
    }
}
