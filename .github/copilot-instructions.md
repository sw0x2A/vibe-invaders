# Vibe Invaders - GitHub Copilot Instructions

## Project Overview
Vibe Invaders is a Space Invaders clone implemented in **Rust 2024 edition** using the **Bevy 0.17** game engine, following Entity Component System (ECS) architecture principles. The project is designed for **WASM (WebAssembly) deployment only**.

## Core Principles
- **Minimal Changes**: Make the smallest possible changes to accomplish tasks
- **No Breaking Changes**: Do not modify working code unless absolutely necessary
- **ECS First**: Follow Entity Component System pattern consistently
- **WASM-Only**: All code must be compatible with WASM target
- **Test with Trunk**: Always verify changes with `trunk serve`

## Code Quality Standards
- Use Rust 2024 edition features and idioms
- Follow standard Rust formatting (rustfmt)
- Keep components as simple data structures
- Systems should be pure functions operating on component queries
- Avoid unnecessary allocations in hot paths (per-frame systems)
- Ensure all code is WASM-compatible

## Development Workflow
1. **Check First**: Run `cargo check --target wasm32-unknown-unknown` before making changes
2. **Build WASM**: Use `trunk build --release` to build the WASM bundle
3. **Test Locally**: Use `trunk serve` to run the game in browser at http://127.0.0.1:8080
4. **Update Documentation**: Keep README.md and ARCHITECTURE.md in sync with significant changes

## Project Structure
```
src/
├── main.rs            # App setup, plugin configuration, system registration
├── components.rs      # Component definitions (Player, Enemy, Bullet, etc.)
├── constants.rs       # Game constants (sizes, speeds, window dimensions)
├── resources.rs       # Resource definitions (GameState)
└── systems/           # System implementations organized by category
    ├── setup.rs       # Initialization systems
    ├── player.rs      # Player movement and shooting
    ├── enemy.rs       # Enemy movement and shooting
    ├── bullet.rs      # Bullet movement and cleanup
    ├── collision.rs   # Collision detection
    └── ui.rs          # UI updates
index.html             # HTML template with canvas element and CSS
Trunk.toml             # Trunk build configuration
```

## Key Technologies
- **Rust Edition**: 2024
- **Bevy Version**: 0.17 (with minimal feature flags for WASM)
- **Target Platform**: WASM (wasm32-unknown-unknown)
- **Build Tool**: Trunk
- **Dependencies**: `bevy = "0.17"`, `rand = "0.9"`

## WASM Canvas Configuration
The game uses a specific canvas element for browser rendering:
- Canvas ID: `#bevy-canvas` (defined in `index.html`)
- Window config includes `canvas: Some("#bevy-canvas".to_string())`
- `fit_canvas_to_parent: true` for responsive sizing
- `prevent_default_event_handling: true` for proper keyboard input

## Common Tasks

### Adding a Component
1. Define in `src/components.rs` with `#[derive(Component)]`
2. Use in systems via queries
3. Update ARCHITECTURE.md if significant

### Adding a System
1. Create in appropriate `src/systems/*.rs` file
2. Register in `src/main.rs` with `.add_systems()`
3. Use appropriate schedule (Startup or Update)

### Modifying Game Constants
- Update values in `src/constants.rs`
- Test gameplay impact with `trunk serve`

### Updating Canvas/HTML
- Modify `index.html` for canvas styling and HTML structure
- Update CSS for responsive behavior

## Bevy-Specific Guidelines
- Use `Query<>` to access entities with specific components
- Use `Commands` to spawn/despawn entities
- Use `Res<>` and `ResMut<>` for global resources
- Components must derive `Component` macro
- Resources must derive `Resource` macro
- Systems are automatically parallelized - avoid shared mutable state

## Testing
Currently, the project does not have automated tests. When adding tests:
- Place unit tests in the same file as the code they test
- Place integration tests in a `tests/` directory
- Follow Bevy's testing patterns for ECS systems

## Performance Considerations
- The game uses optimized dev build settings (opt-level = 1 for dev, 3 for dependencies)
- Always test with `trunk build --release` for accurate performance assessment
- Bevy automatically parallelizes independent systems
- Pre-allocate vectors with `with_capacity()` when size is known
- WASM uses WebGL2 which has some limitations compared to native

## Documentation Files
- **README.md**: User-facing documentation, features, build instructions, controls
- **ARCHITECTURE.md**: Technical documentation of ECS implementation, code structure
- **GAME_FEATURES.md**: Detailed list of game features and mechanics

## Additional Instructions
For more detailed, context-specific instructions, see:
- `.github/instructions/rust.instructions.md` - Rust programming guidelines
- `.github/instructions/bevy-ecs.instructions.md` - Bevy ECS architecture patterns
- `.github/instructions/documentation.instructions.md` - Documentation standards
- `.github/instructions/general.instructions.md` - General repository guidelines

## Security
- Check dependencies for vulnerabilities using `cargo audit`
- Never commit secrets or credentials
- Use `Result` types for fallible operations
- Avoid `.unwrap()` in production code paths

## Key Game Features
- Player-controlled ship with arrow keys or A/D
- Shooting with SPACE key
- 5x11 enemy formation that moves horizontally
- Enemy AI with random shooting
- Collision detection for bullets and enemies
- Score tracking (10 points per enemy)
- Game over conditions (enemies reach bottom or player hit)

## Getting Help
- Check ARCHITECTURE.md for ECS pattern details
- Review existing systems for implementation examples
- Bevy 0.17 documentation: https://docs.rs/bevy/0.17/
- Rust 2024 edition guide: https://doc.rust-lang.org/edition-guide/
