# Pong Game

<span style="color:blue">some *blue* This Pong game is a modern implementation of the classic arcade game, developed using the Rust programming language and the Macroquad framework. It features simple gameplay where two players control paddles to hit a ball back and forth. The goal is to outmaneuver your opponent and score points by getting the ball past their paddle.</span>

## Features

Two Player Mode: Control paddles using keyboard inputs (W and S for the left paddle, Up and Down arrows for the right paddle).

Scoring System: Points are scored when the ball passes one player's paddle, with scores displayed at the top of the screen.

Ball Reset: When a point is scored, the ball resets to the center with a random direction.

Collision Detection: The ball changes direction upon hitting paddles or screen boundaries.

Simple Graphics: Utilizes basic shapes, colors and sprites.

Simple Sound system: Background music and sound effects.

Developer Tools: Basic egui integration for debugging.

## Technical Details

Language and Framework: Rust with Macroquad.

Paddle and Ball Mechanics: Paddles and the ball are defined with their respective properties and behaviors. Paddles can move vertically within the screen limits, and the ball's movement includes bouncing off the screen edges and paddles.

Game Loop: The main game loop handles input processing, game logic updates, and rendering.

FPS: Currently set to 60 but can be changed.

## Code Structure

main.rs: Contains the game loop and rendering logic.

paddle.rs: Defines the Paddle struct and its methods for movement and drawing.

ball.rs: Defines the Ball struct with methods for movement, drawing, collision detection, and resetting.

sounds.rs: Defines sound system.

graphics.rs: Defines graphic system.

egui_menu.rs: Contains code for egui implementation.

constants.rs: Stores all game-related constants like dimensions, speeds and colors.

## Dependencies

macroquad: For game rendering and event handling.

rand: For random value generation.

egui_macroquad: For adding gui debug menu. *NOTE*: You have to downgrade macroquad version to 3.x since egui_macroquad apparently does not seem to work with 4.x version of macroquad.
