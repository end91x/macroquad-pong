# Pong Game

This Pong game is a modern implementation of the classic arcade game, developed using the Rust programming language and the Macroquad framework. It features simple gameplay where two players control paddles to hit a ball back and forth. The goal is to outmaneuver your opponent and score points by getting the ball past their paddle.

## Features

Two Player Mode: Control paddles using keyboard inputs (W and S for the left paddle, Up and Down arrows for the right paddle).

Scoring System: Points are scored when the ball passes one player's paddle, with scores displayed at the top of the screen.

Ball Reset: When a point is scored, the ball resets to the center with a random direction.

Collision Detection: The ball changes direction upon hitting paddles or screen boundaries.

Simple Graphics: Utilizes basic shapes and colors for a clear and visually appealing interface.

## Technical Details

Language and Framework: Rust with Macroquad.

Paddle and Ball Mechanics: Paddles and the ball are defined with their respective properties and behaviors. Paddles can move vertically within the screen limits, and the ball's movement includes bouncing off the screen edges and paddles.

Game Loop: The main game loop handles input processing, game logic updates, and rendering.

## Code Structure

main.rs: Contains the game loop and rendering logic.

paddle.rs: Defines the Paddle struct and its methods for movement and drawing.

ball.rs: Defines the Ball struct with methods for movement, drawing, collision detection, and resetting.

constants.rs: Stores all game-related constants like dimensions and speeds.

## Dependencies

macroquad: For game rendering and event handling.

rand: For random value generation.

This game is a fun project for those interested in game development with Rust and provides a base for more complex game mechanics and features.