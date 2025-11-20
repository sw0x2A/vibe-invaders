# Game Features

## Current Implementation

### Gameplay Mechanics
- **Player Ship**: Green square at the bottom of the screen
- **Movement**: Smooth left/right movement with keyboard controls
- **Shooting**: Unlimited ammunition, fire upward bullets
- **Enemy Formation**: 5 rows × 11 columns (55 enemies total)
- **Enemy AI**: 
  - Move horizontally as a group
  - Descend when reaching screen edges
  - Random shooting every 2 seconds
- **Collision Detection**: Bullet-enemy and bullet-player collisions
- **Score System**: 10 points per destroyed enemy
- **Game Over**: Triggered when enemies reach the player level or player is hit

### Visual Elements
- **Colors**:
  - Player: Green (#00FF00)
  - Enemies: Red (#FF0000)
  - Player Bullets: Yellow (#FFFF00)
  - Enemy Bullets: Orange (#FF8000)
  - Background: Black
  - Text: White
- **Window**: 800×600 pixels, non-resizable
- **UI**: Score counter in top-left corner

### Controls Summary
| Key | Action |
|-----|--------|
| Left Arrow / A | Move Left |
| Right Arrow / D | Move Right |
| Space | Shoot |

### Technical Specifications
- **Player Speed**: 300 units/second
- **Bullet Speed**: 400 units/second
- **Enemy Speed**: 50 units/second
- **Enemy Descent**: 20 pixels per direction change
- **Firing Rate**: Player - on demand, Enemies - every 2 seconds (random)
- **Collision Radius**: Half the sum of entity sizes

## Potential Enhancements

### Gameplay Improvements
1. **Lives System**: Give player 3 lives instead of instant game over
2. **Multiple Levels**: Progress through increasingly difficult stages
3. **Difficulty Progression**: Speed up enemies as player advances
4. **Power-ups**: Special weapons, shields, or temporary invincibility
5. **UFO Bonus Ship**: Periodic UFO that flies across the top for bonus points
6. **Shields/Barriers**: Destructible barriers between player and enemies
7. **Different Enemy Types**: Varying point values and behaviors
8. **Wave Patterns**: Enemies move in more complex patterns
9. **Boss Fights**: Special enemy at end of levels

### Visual & Audio
10. **Particle Effects**: Explosions when enemies are destroyed
11. **Sprite Graphics**: Replace colored squares with pixel art
12. **Background**: Starfield or space-themed background
13. **Animations**: Moving tentacles on enemies, ship exhaust
14. **Sound Effects**: Shooting, explosions, enemy movement
15. **Background Music**: Retro-style game music
16. **Screen Shake**: Impact feedback on explosions

### UI/UX
17. **Main Menu**: Title screen with options
18. **Pause Function**: Pause/resume gameplay
19. **High Score**: Persistent storage of best scores
20. **Game Over Screen**: Display final score with restart option
21. **Wave Counter**: Show current level/wave number
22. **Lives Display**: Visual indicator of remaining lives
23. **FPS Counter**: Development/debug information

### Technical Enhancements
24. **Configurable Settings**: Resolution, controls, volume
25. **Save/Load System**: Save game progress
26. **Replay System**: Record and replay gameplay
27. **Leaderboard**: Online high scores
28. **Controller Support**: Gamepad input
29. **Mobile Touch Controls**: Touchscreen support
30. **Difficulty Settings**: Easy, Normal, Hard modes

## Classic Space Invaders Features Not Yet Implemented
- Enemy animation (tentacles moving up/down)
- Sound effects (iconic "boom-boom-boom-boom" rhythm that speeds up)
- Shields that gradually erode from bullet impacts
- UFO mothership that appears randomly
- Different enemy types with different point values
- Enemy formation gets faster as you destroy more enemies
- Enemies reach bottom = automatic game over (currently just prints message)

## Code Quality & Best Practices
✅ ECS architecture properly implemented
✅ Clean separation of concerns
✅ No compiler warnings
✅ Passes clippy linting
✅ Formatted with cargo fmt
✅ Comprehensive documentation
✅ No security vulnerabilities
✅ Efficient collision detection
✅ Proper resource cleanup (bullets despawn when off-screen)
