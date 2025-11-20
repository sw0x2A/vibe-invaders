# Architecture Overview

## ECS Pattern in Vibe Invaders

This document explains how the Entity Component System (ECS) pattern is implemented in the game using **Bevy 0.17** and **Rust 2024 edition**.

## Code Organization

The codebase is organized into separate modules for better maintainability:

```
src/
├── main.rs            # App setup, plugin configuration, system registration
├── components.rs      # All component definitions (Player, Enemy, Bullet, Star, etc.)
├── constants.rs       # Game constants (sizes, speeds, window dimensions, star/explosion params)
├── resources.rs       # Resource definitions (GameState, GameTextures, GameAudio)
└── systems/           # System implementations organized by category
    ├── mod.rs         # Module exports
    ├── setup.rs       # Initialization systems (spawn entities, camera, load assets)
    ├── player.rs      # Player movement and shooting systems
    ├── enemy.rs       # Enemy movement, shooting, and game over checks
    ├── bullet.rs      # Bullet movement and cleanup systems
    ├── collision.rs   # Collision detection systems with explosion and sound effects
    ├── background.rs  # Starfield animation (spawn and move stars)
    ├── explosion.rs   # Explosion particle effects
    ├── screens.rs     # Start screen, game over screen
    ├── victory.rs     # Victory condition checks
    └── ui.rs          # UI update systems (score display)
```

This modular structure provides:
- **Clear separation of concerns**: Each file has a specific responsibility
- **Easy navigation**: Find related code quickly
- **Better maintainability**: Changes are localized to relevant modules
- **Scalability**: Easy to add new features without cluttering existing files

### Entities
Entities are created by spawning bundles in Bevy:
- **Player Ship**: A single entity with Player component (blue triangle sprite)
- **Enemy Ships**: 55 entities (5 rows × 11 columns) with Enemy component (3 types with different colors)
- **Player Bullets**: Created when player shoots, with Bullet component (cyan)
- **Enemy Bullets**: Created by random enemies, with EnemyBullet component (red)
- **Stars**: Continuously spawned for animated background, with Star component
- **Explosion Particles**: Created on destruction, with ExplosionParticle component
- **UI Elements**: Score display text entity
- **Audio Players**: Temporary entities for sound playback

### Components
Components are pure data structures attached to entities:

```rust
Player              // Marker for player entity
Enemy               // Stores enemy data (original_x position)
Bullet              // Marker for player bullets
EnemyBullet         // Marker for enemy bullets
Velocity            // Movement data (x, y velocity)
Score               // Marker for score UI
Star                // Starfield star data (distance_from_center)
ExplosionParticle   // Explosion particle data (lifetime, max_lifetime)
```

Additionally, Bevy built-in components:
- `Transform`: Position, rotation, scale
- `Sprite`: Visual representation with color and size
- `Text`: UI text rendering
- `AudioPlayer`: Audio playback

### Systems
Systems are functions that operate on entities with specific components:

#### Initialization Systems (Startup)
- `setup`: Creates camera, loads textures and audio assets
- `spawn_player`: Creates player entity and score UI
- `spawn_enemies`: Creates enemy formation (3 different types)

#### Update Systems (Every Frame)
- `spawn_stars`: Spawns stars from center at regular intervals
- `move_stars`: Moves stars outward, updates size and brightness based on distance
- `update_explosions`: Animates explosion particles with fade-out
- `player_movement`: Reads keyboard input, updates Player Transform
- `player_shoot`: Spawns Bullet entities on Space key with sound effect
- `move_bullets`: Updates Bullet Transform based on Velocity
- `move_enemy_bullets`: Updates EnemyBullet Transform
- `move_enemies`: Updates Enemy positions, handles direction changes
- `enemy_shoot`: Randomly spawns EnemyBullet from enemies with sound effect
- `check_bullet_enemy_collision`: Detects hits, spawns explosions, plays sound, updates score
- `check_bullet_player_collision`: Detects player damage, spawns explosion, plays sound
- `check_enemy_reached_bottom`: Game over condition
- `check_all_enemies_destroyed`: Victory condition
- `update_score_display`: Updates score Text
- `cleanup_offscreen_bullets`: Removes bullets off-screen

### Resources
Resources are global data accessible to all systems:

```rust
GameState {
    score: u32,              // Player's current score
    enemy_direction: f32,    // Direction enemies are moving (1.0 or -1.0)
    enemy_shoot_timer: f32,  // Timer for enemy shooting
    star_spawn_timer: f32,   // Timer for star spawning
    victory: bool,           // Victory state flag
}

GameTextures {
    player: Handle<Image>,        // Player sprite
    enemy1: Handle<Image>,        // Enemy type 1 sprite (purple)
    enemy2: Handle<Image>,        // Enemy type 2 sprite (orange)
    enemy3: Handle<Image>,        // Enemy type 3 sprite (green)
    bullet: Handle<Image>,        // Player bullet sprite (cyan)
    enemy_bullet: Handle<Image>,  // Enemy bullet sprite (red)
}

GameAudio {
    player_shoot: Handle<AudioSource>,      // Player shooting sound
    enemy_shoot: Handle<AudioSource>,       // Enemy shooting sound
    enemy_destroyed: Handle<AudioSource>,   // Enemy explosion sound
    player_destroyed: Handle<AudioSource>,  // Player explosion sound
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
