# Pong Game 🦀

This Pong game is a modern implementation of the classic arcade game, developed using the Rust programming language and the Macroquad framework. It features simple gameplay where two players control paddles to hit a ball back and forth. The goal is to outmaneuver your opponent and score points by getting the ball past their paddle.

## Features

1. Two Player Mode: Control paddles using keyboard inputs (W and S for the left paddle, Up and Down arrows for the right paddle).

2. Scoring System: Points are scored when the ball passes one player's paddle, with scores displayed at the top of the screen.

3. Ball Reset: When a point is scored, the ball resets to the center with a random direction.

4. Collision Detection: The ball changes direction upon hitting paddles or screen boundaries.

5. Simple Graphics: Utilizes basic shapes, colors and usage of sprites.
Simple Graphics: Utilizes basic shapes, colors and sprites.

6. Simple Sound system: Background music and sound effects.

7. Developer Tools: Basic egui integration for debugging.

## Technical Details

1. Language and Framework: Rust with Macroquad.

2. Paddle and Ball Mechanics: Paddles and the ball are defined with their respective properties and behaviors. Paddles can move vertically within the screen limits, and the ball's movement includes bouncing off the screen edges and paddles.

3. Game Loop: The main game loop handles input processing, game logic updates, and rendering.

4. FPS: Currently set to 60 but can be changed.

## Code Structure

1. main.rs: Contains the game loop and rendering logic.

2. paddle.rs: Defines the Paddle struct and its methods for movement and drawing.

3. ball.rs: Defines the Ball struct with methods for movement, drawing, collision detection, and resetting.

4. egui_menu.rs: Contains code for egui implementation.

5. constants.rs: Stores all game-related constants like dimensions, speeds and colors.
sounds.rs: Defines sound system.

6. graphics.rs: Defines graphic system.

7. egui_menu.rs: Contains code for egui implementation.

## Dependencies

1. macroquad: For game rendering, sounds and event handling.

2. rand: For random value generation.

3. egui_macroquad: For adding gui debug menu. **NOTE**: You have to downgrade macroquad version to 3.x since egui_macroquad apparently does not seem to work with 4.x version of macroquad.

## Screenshots

![pong_1](https://github.com/end91x/macroquad-pong/assets/160602132/3efdce82-3d86-4ea8-a183-e74cb5dbbd26)

