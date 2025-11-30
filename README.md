# vibe-invaders

> **🤖 100% Vibe-Coded, AI-Generated**  
> Everything in this repository has been created entirely by AI using simple, non-technical language prompts. No technical jargon or debug hints were used in the development process.

**🎮 [Play the Game](https://vibe-invaders.00101010.org/)**

---

Space Invaders clone in Rust using Bevy engine

## Description
A classic Space Invaders game implemented in Rust using the Bevy game engine, following Entity Component System (ECS) architecture principles.

Built with **Rust 2024 edition** and **Bevy 0.17**, designed specifically for **WASM (WebAssembly)** deployment.

## Features
- **Player-controlled ship**: Move left/right with arrow keys or A/D
- **Shooting mechanics**: Press SPACE to shoot
- **Animated starfield background**: Dynamic stars moving from center to edges, creating a high-speed flight effect
- **Colorful comic-style graphics**: Enhanced visuals with detailed, colorful sprites
- **Enemy invaders**: 5 rows × 11 columns of enemies that move back and forth (3 different types)
- **Enemy AI**: Enemies move horizontally and descend when hitting screen edges
- **Enemy shooting**: Enemies randomly shoot projectiles at the player
- **Collision detection**: Bullets destroy enemies and enemy bullets can hit the player
- **Explosion animations**: Particle-based explosions when enemies or player are destroyed
- **Sound effects**: Audio feedback for shooting and destruction
- **Score tracking**: Earn 10 points per destroyed enemy
- **Game over conditions**: When enemies reach the bottom or player is hit

## Controls
- **Arrow Keys** or **A/D**: Move left/right
- **SPACE**: Shoot

## Building and Running (WASM)

This project is designed for WASM deployment only.

### Prerequisites
- Rust (latest stable version)
- Cargo
- Rust WASM target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Trunk (WASM bundler):
  ```bash
  cargo install --locked trunk
  ```

### Build for Web
```bash
trunk build --release
```

### Run Locally
```bash
trunk serve
```
Then open http://127.0.0.1:8080 in your browser.

### GitHub Pages Deployment

The game is automatically deployed to GitHub Pages when changes are pushed to the `main` branch. The deployment workflow builds the WASM bundle and publishes it to GitHub Pages.

## Architecture
The game follows ECS (Entity Component System) principles using Bevy 0.17.

### Code Structure
```
src/
├── main.rs            # App setup and configuration
├── components.rs      # Component definitions
├── constants.rs       # Game constants
├── resources.rs       # Resource definitions (GameState, GameTextures, GameAudio)
└── systems/           # System implementations
    ├── setup.rs       # Initialization systems
    ├── player.rs      # Player movement and shooting
    ├── enemy.rs       # Enemy movement and shooting
    ├── bullet.rs      # Bullet movement and cleanup
    ├── collision.rs   # Collision detection
    ├── background.rs  # Starfield animation
    ├── explosion.rs   # Explosion particle effects
    └── ui.rs          # UI updates
```

For detailed architecture documentation, see [ARCHITECTURE.md](ARCHITECTURE.md).

### Quick Overview

**Components**: Player, Enemy, Bullet, EnemyBullet, Velocity, Score, Star, ExplosionParticle

**Systems**: player_movement, player_shoot, move_bullets, move_enemies, enemy_shoot, collision detection, spawn_stars, move_stars, update_explosions, score updates, cleanup

**Resources**: GameState (tracks score, enemy direction, timers), GameTextures (sprite assets), GameAudio (sound assets)

## Technical Details
- Rust Edition: 2024
- Bevy Version: 0.17
- Target Platform: WASM (WebAssembly)
- Window size: 800×600
- Player speed: 300 units/second
- Bullet speed: 400 units/second
- Enemy speed: 50 units/second
- Enemy formation: 5 rows × 11 columns (3 enemy types)
- Enemy shoot interval: Every 2 seconds (random enemy)
- Star spawn rate: Every 0.05 seconds
- Explosion particles: 20 per explosion
- Audio formats: WAV for sound effects

## WASM Canvas Configuration
The game uses a specific canvas element (`#bevy-canvas`) for proper rendering in browsers. Key configuration:
- Canvas element with ID `bevy-canvas` in `index.html`
- `fit_canvas_to_parent: true` for responsive sizing
- `prevent_default_event_handling: true` for proper keyboard input handling
- Fixed resolution of 800×600 with responsive CSS scaling

## Future Enhancements
- Lives system
- Multiple levels with increasing difficulty
- Power-ups
- Background music
- High score persistence
- Shields/barriers
- UFO bonus ship
- Main menu and pause functionality
