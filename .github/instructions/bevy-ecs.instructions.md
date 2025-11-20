---
applyTo: "src/**/*.rs"
---

# Bevy Game Engine and ECS Architecture Guidelines

## Bevy Version
This project uses **Bevy 0.17**. Always check Bevy 0.17 documentation when adding features.

## Entity Component System (ECS) Pattern

### Components
Components are pure data structures attached to entities:
- Keep components simple and focused on data, not behavior
- Use marker components (zero-sized types) for entity identification
- Store state in components, not in systems
- All components must derive `Component`

**Examples in this project:**
```rust
#[derive(Component)]
struct Player;  // Marker component

#[derive(Component)]
struct Enemy {
    original_x: f32,  // State data
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
```

### Systems
Systems are functions that operate on entities with specific components:
- Systems should be pure functions without side effects (except through Commands and Resources)
- Query for only the components you need
- Use `Added<T>` and `Changed<T>` filters to optimize queries
- Systems are automatically parallelized by Bevy - avoid shared mutable state

**Query patterns:**
```rust
// Read-only query
Query<&Transform, With<Player>>

// Mutable query
Query<&mut Transform, With<Enemy>>

// Multiple components
Query<(Entity, &Transform, &Velocity), With<Bullet>>

// Without filter
Query<&Transform, Without<Player>>
```

### Resources
Resources are global data accessible to all systems:
- Use resources for game-wide state (scores, timers, configuration)
- Minimize resource usage - prefer components when possible
- All resources must derive `Resource`
- Use `Res<T>` for read-only access, `ResMut<T>` for mutable access

### System Scheduling
- **Startup systems**: Run once at app initialization (use `.add_systems(Startup, ...)`)
- **Update systems**: Run every frame (use `.add_systems(Update, ...)`)
- Bevy handles system ordering based on data dependencies
- Explicit ordering with `.after()` or `.before()` when needed

## Bevy Built-in Components
Commonly used Bevy components in this project:
- `Transform` - Position, rotation, scale (3D)
- `Sprite` - Visual representation (2D)
- `Text` - UI text rendering
- `Camera2d` - 2D camera

## Spawning Entities
Use bundles for better organization:
```rust
commands.spawn((
    Sprite { /* sprite config */ },
    Transform::from_xyz(x, y, 0.0),
    Player,  // Custom component
));
```

## Despawning Entities
Always clean up entities properly:
```rust
commands.entity(entity_id).despawn();
```

## Input Handling
Use `Res<ButtonInput<KeyCode>>` for keyboard input:
```rust
fn system(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.pressed(KeyCode::Space) {
        // Handle input
    }
}
```

## Time Management
Use `Res<Time>` for frame-independent behavior:
```rust
fn system(time: Res<Time>) {
    let delta = time.delta_secs();
    // Update positions/timers based on delta
}
```

## Best Practices
1. **Separation of concerns**: Keep data (components) separate from logic (systems)
2. **Composability**: Design components to be reusable (e.g., Velocity works for all bullet types)
3. **Single responsibility**: Each system should have one clear purpose
4. **Query efficiency**: Filter queries to minimize unnecessary iteration
5. **Avoid entity lookup by name/tag**: Use marker components and queries instead
6. **Entity lifecycle**: Spawn in setup systems, despawn in cleanup systems or when no longer needed

## Debugging
- Use `println!` or Bevy's `info!`, `warn!`, `error!` logging macros
- Query entity counts to verify spawning/despawning
- Use Bevy's diagnostic plugins for performance monitoring

## Adding New Features
When adding a new feature (example: shields):

1. **Define component** in `src/components.rs`:
```rust
#[derive(Component)]
pub struct Shield {
    pub health: u32,
}
```

2. **Create system** in appropriate `src/systems/*.rs`:
```rust
pub fn check_bullet_shield_collision(/* ... */) {
    // Implementation
}
```

3. **Register system** in `src/main.rs`:
```rust
.add_systems(Update, check_bullet_shield_collision)
```

4. **Update documentation** in ARCHITECTURE.md if the feature is significant
