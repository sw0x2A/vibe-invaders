use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Spawn stars from the center of the screen
pub fn spawn_stars(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    game_state.star_spawn_timer += time.delta_secs();

    if game_state.star_spawn_timer >= STAR_SPAWN_INTERVAL {
        game_state.star_spawn_timer = 0.0;

        let mut rng = rand::thread_rng();
        
        // Random angle for star direction
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        
        // Random speed variation
        let speed = STAR_BASE_SPEED * rng.gen_range(0.5..1.5);
        
        // Spawn star at center with slight random offset
        let offset = rng.gen_range(-10.0..10.0);
        let x = offset * angle.cos();
        let y = offset * angle.sin();

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(STAR_MIN_SIZE, STAR_MIN_SIZE)),
                ..default()
            },
            Transform::from_xyz(x, y, -1.0), // Behind everything
            Star {
                distance_from_center: 0.0,
            },
            Velocity {
                x: speed * angle.cos(),
                y: speed * angle.sin(),
            },
        ));
    }
}

/// Move stars and update their size/brightness
pub fn move_stars(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut Star, &Velocity)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut sprite, mut star, velocity) in query.iter_mut() {
        // Move star
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        // Calculate distance from center
        star.distance_from_center = (transform.translation.x.powi(2) + transform.translation.y.powi(2)).sqrt();

        // Calculate progress based on distance (0.0 at center, 1.0 at max distance)
        let progress = (star.distance_from_center / STAR_MAX_DISTANCE).min(1.0);

        // Increase size as star moves outward
        let size = STAR_MIN_SIZE + (STAR_MAX_SIZE - STAR_MIN_SIZE) * progress;
        sprite.custom_size = Some(Vec2::new(size, size));

        // Increase brightness as star moves outward
        let brightness = STAR_MIN_BRIGHTNESS + (STAR_MAX_BRIGHTNESS - STAR_MIN_BRIGHTNESS) * progress;
        sprite.color = Color::srgb(brightness, brightness, brightness);

        // Despawn if off-screen
        if transform.translation.x.abs() > WINDOW_WIDTH / 2.0 + 10.0
            || transform.translation.y.abs() > WINDOW_HEIGHT / 2.0 + 10.0
        {
            commands.entity(entity).despawn();
        }
    }
}
