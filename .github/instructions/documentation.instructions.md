---
applyTo: "**/*.md"
excludeAgent: "code-review"
---

# Documentation Guidelines

## Documentation Files
- **README.md**: User-facing documentation, features, build instructions, controls
- **ARCHITECTURE.md**: Technical documentation of ECS implementation, code structure
- **GAME_FEATURES.md**: Detailed list of game features and mechanics

## Documentation Standards
- Keep documentation synchronized with code changes
- Use clear, concise language
- Include code examples when explaining technical concepts
- Use markdown formatting consistently

## When to Update Documentation

### README.md
Update when:
- Adding new features that affect gameplay
- Changing build/run instructions
- Modifying controls or key bindings
- Adding new dependencies with special requirements
- Changing system requirements

### ARCHITECTURE.md
Update when:
- Adding new components, systems, or resources
- Restructuring code organization
- Changing ECS patterns or practices
- Adding new modules or files

### GAME_FEATURES.md
Update when:
- Implementing new game mechanics
- Modifying existing gameplay features
- Adding configuration options

## Markdown Style
- Use headers hierarchically (# for title, ## for sections, ### for subsections)
- Use code blocks with language tags (```rust, ```bash, etc.)
- Use bullet points for lists
- Use **bold** for emphasis on important terms
- Use `inline code` for file names, commands, and short code snippets

## Code Examples in Documentation
- Keep examples minimal and focused
- Ensure examples compile and follow project conventions
- Use comments to explain non-obvious code
- Show both declaration and usage when helpful

## Consistency
- Maintain consistent terminology throughout all documentation
- Use "Entity Component System" or "ECS" consistently
- Refer to "Bevy 0.17" consistently
- Use "Rust 2024 edition" when referring to Rust version
