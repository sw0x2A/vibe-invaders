# Changelog

All notable changes to Vibe Invaders will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.23.0] - 2025-11-30

### Added
- AI-generated development banner in README.md
- "Play the Game" CTA link to deployed version
- GitHub repository link in web deployment (top-right corner)
- CHANGELOG.md with semantic versioning history

### Changed
- Updated version to 0.23.0

## [0.22.0] - 2025-11-30

### Fixed
- Improved AudioContext workaround to handle edge cases for browser autoplay policy
- Added JavaScript workaround to fix WASM audio autoplay policy issue

## [0.21.0] - 2025-11-29

### Fixed
- Added webgl2 feature to Bevy to fix WASM rendering in browser

## [0.20.0] - 2025-11-29

### Fixed
- Fixed WASM canvas rendering with explicit canvas targeting
- Removed object-fit from canvas element CSS
- Updated documentation for WASM-only deployment

## [0.19.0] - 2025-11-29

### Fixed
- Fixed WASM canvas rendering with explicit resolution and fit_canvas_to_parent
- Used WINDOW_WIDTH and WINDOW_HEIGHT constants for WASM resolution

## [0.18.0] - 2025-11-29

### Fixed
- Fixed WASM rendering by using Windowed mode instead of BorderlessFullscreen

## [0.17.0] - 2025-11-29

### Added
- WASM support and GitHub Pages deployment workflow
- Loading screen with spinner and timeout
- Trunk build configuration

### Changed
- Addressed code review feedback: removed unused profile and added loading timeout

## [0.16.0] - 2025-11-21

### Fixed
- Fixed text centering in fullscreen mode using Justify::Center
- Removed unused window_dims parameter from setup_start_screen

## [0.15.0] - 2025-11-21

### Added
- Dynamic fullscreen resolution support
- High score table
- Space key delay to prevent accidental restarts

## [0.14.0] - 2025-11-21

### Added
- Fullscreen mode with enhanced starfield
- Manga-style graphics for player and enemies
- Futuristic techno electronic music soundtrack

## [0.13.0] - 2025-11-21

### Fixed
- Added WAV audio format support to fix runtime panic
- Added wav feature to bevy dependencies

## [0.12.0] - 2025-11-20

### Added
- Animated starfield background with parallax effect
- Particle-based explosion effects
- Enhanced colorful comic-style sprites
- Sound effects for shooting and destruction
- Implementation summary documentation

### Changed
- Optimized constant calculations based on code review feedback

## [0.11.0] - 2025-11-20

### Fixed
- Fixed game over screen displaying zero score
- Added victory state when all enemies are destroyed

## [0.10.0] - 2025-11-20

### Added
- Three different enemy types with distinct colors
- Enemy shooting mechanics with random intervals
- Enemy bullet collision detection

## [0.9.0] - 2025-11-20

### Added
- Score tracking system (10 points per enemy)
- Score display UI in top-left corner

## [0.8.0] - 2025-11-20

### Added
- Player-enemy bullet collision detection
- Game over screen with restart functionality

## [0.7.0] - 2025-11-20

### Added
- Bullet-enemy collision detection
- Enemy despawn on hit

## [0.6.0] - 2025-11-20

### Added
- Enemy movement system (horizontal with descent)
- Enemy formation (5 rows × 11 columns)

## [0.5.0] - 2025-11-20

### Added
- Player shooting mechanics
- Bullet movement and cleanup systems

## [0.4.0] - 2025-11-20

### Added
- Player movement with keyboard controls (arrow keys and A/D)
- Player boundaries to keep ship on screen

## [0.3.0] - 2025-11-20

### Added
- Basic player entity with sprite
- Camera setup

## [0.2.0] - 2025-11-20

### Added
- ECS architecture with Bevy 0.17
- Project structure with components, systems, and resources modules

## [0.1.0] - 2025-11-20

### Added
- Initial project setup
- Rust 2024 edition configuration
- Basic Cargo.toml with Bevy dependency
