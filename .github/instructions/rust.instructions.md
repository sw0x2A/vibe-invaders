---
applyTo: "**/*.rs"
---

# Rust Programming Guidelines for Vibe Invaders

## Rust Edition
This project uses **Rust 2024 edition**. Ensure all code follows Rust 2024 idioms and features.

## Code Style
- Follow standard Rust formatting conventions (use `rustfmt`)
- Use meaningful variable and function names that describe their purpose
- Prefer explicit types when it improves code clarity
- Use `const` for compile-time constants in `constants.rs`

## Error Handling
- Currently, the game uses Bevy's panic-based error handling
- When adding fallible operations, prefer `Result` types
- Use `.expect()` with meaningful messages for operations that should never fail
- Avoid `.unwrap()` in production code paths

## Ownership and Borrowing
- Minimize unnecessary clones
- Use references (`&` and `&mut`) appropriately
- Bevy queries handle borrowing automatically - understand query parameter rules
- Components should be simple data structures; avoid complex lifetime management

## Pattern Matching
- Use exhaustive pattern matching where possible
- Leverage Rust's match expressions for clarity
- Use `if let` for single-variant matches
- Consider using destructuring in function parameters when appropriate

## Dependencies
- Keep dependencies minimal and focused
- This project uses:
  - `bevy = "0.17"` - Game engine (with minimal features enabled)
  - `rand = "0.8"` - Random number generation for enemy shooting
- When adding dependencies, check for security vulnerabilities using `cargo audit`

## Bevy-Specific Rust Patterns
- Systems are functions that query entities
- Use `Query<>` to access entities with specific components
- Use `Commands` to spawn/despawn entities
- Use `Res<>` and `ResMut<>` for global resources
- Components should derive `Component` macro
- Resources should derive `Resource` macro

## Performance
- Avoid allocations in hot paths (systems that run every frame)
- Use `Vec` pre-allocation with `with_capacity()` when size is known
- Bevy handles most performance optimizations automatically
- Profile before optimizing

## Common Patterns
```rust
// Querying entities
fn system(query: Query<(&ComponentA, &mut ComponentB)>) {
    for (comp_a, mut comp_b) in query.iter() {
        // Use components
    }
}

// Spawning entities
fn setup(mut commands: Commands) {
    commands.spawn((
        ComponentA { /* ... */ },
        ComponentB { /* ... */ },
    ));
}

// Using resources
fn system(mut game_state: ResMut<GameState>) {
    game_state.score += 10;
}
```
