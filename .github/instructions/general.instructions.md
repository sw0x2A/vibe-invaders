# Vibe Invaders - General Repository Instructions

## Project Overview
This is a Space Invaders clone implemented in Rust using the Bevy game engine (version 0.17), following Entity Component System (ECS) architecture principles.

## Code Quality Standards
- **Minimal Changes**: Make the smallest possible changes to accomplish the task
- **No Breaking Changes**: Do not modify working code unless absolutely necessary
- **Rust Edition**: Use Rust 2024 edition features appropriately
- **ECS Pattern**: Follow the Entity Component System pattern consistently
- **Module Organization**: Keep code organized by responsibility (components, systems, resources)

## Development Workflow
1. **Always check first**: Run `cargo check` before making changes to understand current state
2. **Build incrementally**: Test changes with `cargo build` or `cargo check` after modifications
3. **Run the game**: Use `cargo run --release` to test gameplay changes
4. **Documentation**: Update relevant documentation (README.md, ARCHITECTURE.md) when making significant changes

## File Organization
- `src/main.rs` - App setup, plugin configuration, system registration
- `src/components.rs` - Component definitions (Player, Enemy, Bullet, etc.)
- `src/constants.rs` - Game constants (sizes, speeds, window dimensions)
- `src/resources.rs` - Resource definitions (GameState)
- `src/systems/` - System implementations organized by category

## Testing
Currently, the project does not have automated tests. When adding tests:
- Place unit tests in the same file as the code they test
- Place integration tests in a `tests/` directory
- Follow Bevy's testing patterns for ECS systems

## Performance Considerations
- The game uses optimized dev build settings (opt-level = 1 for dev, 3 for dependencies)
- Always test in release mode for accurate performance assessment
- Bevy automatically parallelizes independent systems

## Common Tasks
- **Adding a component**: Define in `components.rs`, use in systems
- **Adding a system**: Create in appropriate `systems/*.rs` file, register in `main.rs`
- **Adding a resource**: Define in `resources.rs`, initialize in setup
- **Modifying game constants**: Update values in `constants.rs`
