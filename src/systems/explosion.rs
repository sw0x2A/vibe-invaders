use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;

/// Spawn an explosion at a position
pub fn spawn_explosion(commands: &mut Commands, position: Vec3, color: Color) {
    let mut rng = rand::thread_rng();

    for _ in 0..EXPLOSION_PARTICLES {
        // Random angle for particle direction
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        
        // Random speed variation
        let speed = EXPLOSION_PARTICLE_SPEED * rng.gen_range(0.5..1.5);
        
        // Random particle size
        let size = EXPLOSION_PARTICLE_SIZE * rng.gen_range(0.5..1.5);
        
        // Slight color variation
        let color_variation = rng.gen_range(0.8..1.2);
        let base_color = color.to_srgba();
        let particle_color = Color::srgb(
            (base_color.red * color_variation).min(1.0),
            (base_color.green * color_variation).min(1.0),
            (base_color.blue * color_variation).min(1.0),
        );

        commands.spawn((
            Sprite {
                color: particle_color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, position.z + 0.1),
            ExplosionParticle {
                lifetime: 0.0,
                max_lifetime: EXPLOSION_LIFETIME * rng.gen_range(0.8..1.2),
            },
            Velocity {
                x: speed * angle.cos(),
                y: speed * angle.sin(),
            },
        ));
    }
}

/// Update explosion particles
pub fn update_explosions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut ExplosionParticle, &Velocity)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut sprite, mut particle, velocity) in query.iter_mut() {
        // Update lifetime
        particle.lifetime += time.delta_secs();

        // Move particle
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        // Fade out based on lifetime
        let alpha = 1.0 - (particle.lifetime / particle.max_lifetime);
        let current_color = sprite.color.to_srgba();
        sprite.color = Color::srgba(
            current_color.red,
            current_color.green,
            current_color.blue,
            alpha,
        );

        // Despawn if lifetime exceeded
        if particle.lifetime >= particle.max_lifetime {
            commands.entity(entity).despawn();
        }
    }
}
