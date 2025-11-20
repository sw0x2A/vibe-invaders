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
The game follows ECS (Entity Component System) principles using Bevy.

For detailed architecture documentation, see [ARCHITECTURE.md](ARCHITECTURE.md).

### Quick Overview

**Components**: Player, Enemy, Bullet, EnemyBullet, Velocity, Score

**Systems**: player_movement, player_shoot, move_bullets, move_enemies, enemy_shoot, collision detection, score updates, cleanup

**Resources**: GameState (tracks score, enemy direction, timers)

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
