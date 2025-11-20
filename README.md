# vibe-invaders
Space Invaders clone in Rust using Bevy engine

## Description
A classic Space Invaders game implemented in Rust using the Bevy game engine, following Entity Component System (ECS) architecture principles.

## Features
- **Player-controlled ship**: Move left/right with arrow keys or A/D
- **Shooting mechanics**: Press SPACE to shoot
- **Enemy invaders**: 5 rows × 11 columns of enemies that move back and forth
- **Enemy AI**: Enemies move horizontally and descend when hitting screen edges
- **Enemy shooting**: Enemies randomly shoot projectiles at the player
- **Collision detection**: Bullets destroy enemies and enemy bullets can hit the player
- **Score tracking**: Earn 10 points per destroyed enemy
- **Game over conditions**: When enemies reach the bottom or player is hit

## Controls
- **Arrow Keys** or **A/D**: Move left/right
- **SPACE**: Shoot

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

## Architecture
The game follows ECS (Entity Component System) principles using Bevy:

### Components
- `Player`: Player ship component
- `Enemy`: Enemy invader component with position tracking
- `Bullet`: Player bullet component
- `EnemyBullet`: Enemy bullet component
- `Velocity`: Movement velocity component
- `Score`: Score display component

### Systems
- `player_movement`: Handles player ship movement
- `player_shoot`: Creates bullets when player shoots
- `move_bullets`: Updates bullet positions
- `move_enemy_bullets`: Updates enemy bullet positions
- `move_enemies`: Controls enemy formation movement
- `enemy_shoot`: Randomly selects enemies to shoot
- `check_bullet_enemy_collision`: Detects and handles player bullet hits
- `check_bullet_player_collision`: Detects and handles enemy bullet hits
- `check_enemy_reached_bottom`: Checks for game over condition
- `update_score_display`: Updates score UI
- `cleanup_offscreen_bullets`: Removes bullets that leave the screen

### Resources
- `GameState`: Tracks score, enemy direction, and shooting timers

## Technical Details
- Window size: 800×600
- Player speed: 300 units/second
- Bullet speed: 400 units/second
- Enemy speed: 50 units/second
- Enemy formation: 5 rows × 11 columns
- Enemy shoot interval: Every 2 seconds (random enemy)

## Future Enhancements
- Lives system
- Multiple levels with increasing difficulty
- Power-ups
- Sound effects and music
- High score persistence
- Shields/barriers
- UFO bonus ship
- Particle effects
- Main menu and pause functionality
