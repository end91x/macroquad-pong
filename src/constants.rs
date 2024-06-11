use macroquad::prelude::*;

/// Constants
pub const PADDLE_W: f32 = 20f32;
pub const PADDLE_H: f32 = 80f32;
pub const PADDLE_SPEED: f32 = 9f32;
pub const PADDLE_BOUNCE_ANGLE_FACTOR: f32 = 0.3;

pub const BALL_RADIUS: f32 = 15f32;
pub const BALL_SPEED: f32 = 6f32;
pub const BALL_SPEED_MULTIPLIER: f32 = 1.05;
pub const MAX_BALL_SPEED: f32 = 2f32;

pub const BG_COLOR: Color = Color::new(0.19, 0.69, 0.30, 1.0);
pub const PADDLE_COLOR: Color = WHITE;
pub const BALL_COLOR: Color = WHITE;
