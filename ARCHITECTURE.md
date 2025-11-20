# Architecture Overview

## ECS Pattern in Vibe Invaders

This document explains how the Entity Component System (ECS) pattern is implemented in the game.

### Entities
Entities are created by spawning bundles in Bevy:
- **Player Ship**: A single entity with Player component
- **Enemy Ships**: 55 entities (5 rows × 11 columns) with Enemy component
- **Player Bullets**: Created when player shoots, with Bullet component
- **Enemy Bullets**: Created by random enemies, with EnemyBullet component
- **UI Elements**: Score display text entity

### Components
Components are pure data structures attached to entities:

```rust
Player              // Marker for player entity
Enemy               // Stores enemy data (original_x position)
Bullet              // Marker for player bullets
EnemyBullet         // Marker for enemy bullets
Velocity            // Movement data (x, y velocity)
Score               // Marker for score UI
```

Additionally, Bevy built-in components:
- `Transform`: Position, rotation, scale
- `Sprite`: Visual representation
- `Text`: UI text rendering

### Systems
Systems are functions that operate on entities with specific components:

#### Initialization Systems (Startup)
- `setup`: Creates camera
- `spawn_player`: Creates player entity and score UI
- `spawn_enemies`: Creates enemy formation

#### Update Systems (Every Frame)
- `player_movement`: Reads keyboard input, updates Player Transform
- `player_shoot`: Spawns Bullet entities on Space key
- `move_bullets`: Updates Bullet Transform based on Velocity
- `move_enemy_bullets`: Updates EnemyBullet Transform
- `move_enemies`: Updates Enemy positions, handles direction changes
- `enemy_shoot`: Randomly spawns EnemyBullet from enemies
- `check_bullet_enemy_collision`: Detects hits, updates score
- `check_bullet_player_collision`: Detects player damage
- `check_enemy_reached_bottom`: Game over condition
- `update_score_display`: Updates score Text
- `cleanup_offscreen_bullets`: Removes bullets off-screen

### Resources
Resources are global data accessible to all systems:

```rust
GameState {
    score: u32,              // Player's current score
    enemy_direction: f32,    // Direction enemies are moving (1.0 or -1.0)
    enemy_shoot_timer: f32,  // Timer for enemy shooting
}
```

### Data Flow

```
Keyboard Input → player_movement → Transform
              → player_shoot → Spawn Bullet

Timer → enemy_shoot → Spawn EnemyBullet

Velocity + Time → move_bullets → Transform
                → move_enemy_bullets → Transform

Enemies + Time → move_enemies → Transform

Bullet + Enemy → check_bullet_enemy_collision → Despawn + Update Score

EnemyBullet + Player → check_bullet_player_collision → Game Over

GameState → update_score_display → Text

Transform → cleanup_offscreen_bullets → Despawn
```

### Key ECS Benefits in This Implementation

1. **Separation of Concerns**: Components are data, Systems are logic
2. **Composability**: Velocity component is reused for different bullet types
3. **Performance**: Bevy's query system efficiently processes entities
4. **Maintainability**: Easy to add new features (e.g., power-ups would be new components/systems)
5. **Parallelization**: Bevy automatically parallelizes independent systems

### Adding New Features

To add a new feature (e.g., shields):

1. **Create Component**: 
```rust
#[derive(Component)]
struct Shield { health: u32 }
```

2. **Spawn Entity**:
```rust
commands.spawn((
    SpriteBundle { ... },
    Shield { health: 3 },
));
```

3. **Add System**:
```rust
fn check_bullet_shield_collision(
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut shield_query: Query<(Entity, &Transform, &mut Shield)>,
    ...
) {
    // Collision detection and health reduction
}
```

4. **Register System**:
```rust
.add_systems(Update, check_bullet_shield_collision)
```

This demonstrates the power and flexibility of the ECS pattern!
