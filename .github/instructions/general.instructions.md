# Vibe Invaders - General Repository Instructions

## Project Overview
This is a Space Invaders clone implemented in Rust using the Bevy game engine (version 0.17), following Entity Component System (ECS) architecture principles. The project is designed for **WASM (WebAssembly) deployment only**.

## Code Quality Standards
- **Minimal Changes**: Make the smallest possible changes to accomplish the task
- **No Breaking Changes**: Do not modify working code unless absolutely necessary
- **Rust Edition**: Use Rust 2024 edition features appropriately
- **ECS Pattern**: Follow the Entity Component System pattern consistently
- **Module Organization**: Keep code organized by responsibility (components, systems, resources)
- **WASM-First**: All code should be optimized for WASM deployment

## Development Workflow
1. **Always check first**: Run `cargo check --target wasm32-unknown-unknown` before making changes
2. **Build WASM**: Use `trunk build --release` to build the WASM bundle
3. **Test locally**: Use `trunk serve` to run the game in a browser at http://127.0.0.1:8080
4. **Documentation**: Update relevant documentation (README.md, ARCHITECTURE.md) when making significant changes

## File Organization
- `src/main.rs` - App setup, plugin configuration, system registration
- `src/components.rs` - Component definitions (Player, Enemy, Bullet, etc.)
- `src/constants.rs` - Game constants (sizes, speeds, window dimensions)
- `src/resources.rs` - Resource definitions (GameState)
- `src/systems/` - System implementations organized by category
- `index.html` - HTML template with canvas element and CSS styling
- `Trunk.toml` - Trunk build configuration

## WASM Canvas Configuration
The game uses a specific canvas element for proper browser rendering:
- Canvas ID: `#bevy-canvas`
- The canvas property in Window config targets this element
- `fit_canvas_to_parent: true` for responsive sizing
- `prevent_default_event_handling: true` for proper keyboard input

## Testing
Currently, the project does not have automated tests. When adding tests:
- Place unit tests in the same file as the code they test
- Place integration tests in a `tests/` directory
- Follow Bevy's testing patterns for ECS systems

## Performance Considerations
- The game uses optimized dev build settings (opt-level = 1 for dev, 3 for dependencies)
- Always test in release mode with `trunk build --release`
- Bevy automatically parallelizes independent systems
- WASM builds use WebGL2 for rendering

## Common Tasks
- **Adding a component**: Define in `components.rs`, use in systems
- **Adding a system**: Create in appropriate `systems/*.rs` file, register in `main.rs`
- **Adding a resource**: Define in `resources.rs`, initialize in setup
- **Modifying game constants**: Update values in `constants.rs`
- **Updating canvas styling**: Modify `index.html` CSS
