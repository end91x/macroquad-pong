use ::rand::prelude::*;
use macroquad::audio::Sound;
use macroquad::prelude::*;

use crate::constants::*;
use crate::sound::play_collision_sound;

/// Represents a ball
pub struct Ball {
    /// The circle that represents the ball
    pub circle: Circle,
    /// The direction of the ball
    pub dir: Vec2,
}

impl Ball {
    /// Creates a new ball with the given circle
    ///
    /// # Arguments
    ///
    /// * `circle` - The circle that represents the ball
    pub fn new(circle: Circle) -> Self {
        Self {
            circle,
            dir: Self::random_direction(),
        }
    }

    /// Moves the ball in the direction it is moving
    pub fn movement(&mut self) {
        // Move the ball in the direction it is moving
        self.circle.x += self.dir.x * BALL_SPEED;
        self.circle.y += self.dir.y * BALL_SPEED;

        // Bounce the ball off the top or bottom of the screen
        if self.circle.y - BALL_RADIUS <= 0. + 32.
            || self.circle.y >= screen_height() - BALL_RADIUS - 32.
        {
            self.dir.y = -self.dir.y;
        }

        // Clamp the speed of the ball
        self.dir.x = self.dir.x.clamp(-MAX_BALL_SPEED, MAX_BALL_SPEED);
        self.dir.y = self.dir.y.clamp(-MAX_BALL_SPEED, MAX_BALL_SPEED);
    }

    /// Generates a random direction for the ball to move in
    ///
    /// # Returns
    ///
    /// * A Vec2 that represents the direction the ball should move in
    fn random_direction() -> Vec2 {
        let mut rng: ThreadRng = thread_rng();
        let mut dir_x: f32;
        let mut dir_y: f32;

        // Generate a random direction that is not too steep or too shallow
        loop {
            dir_x = rng.gen_range(-1.0..1.0);
            dir_y = rng.gen_range(-1.0..1.0);

            if dir_x.abs() > 0.8 && dir_y.abs() > 0.2 {
                break;
            }
        }

        // Normalize again to ensure it's a unit vector after adjustments
        vec2(dir_x, dir_y).normalize()
    }

    /// Checks for collisions between the ball and the paddles
    ///
    /// # Arguments
    ///
    /// * `paddle_1` - The first paddle rect
    /// * `paddle_2` - The second paddle rect
    pub async fn collision_with_paddles(&mut self, paddle_1: &Rect, paddle_2: &Rect, sound: Sound) {
        let ball_rect: Rect = Rect::new(
            self.circle.x - BALL_RADIUS,
            self.circle.y - BALL_RADIUS,
            2. * BALL_RADIUS,
            2. * BALL_RADIUS,
        );

        // Check for collisions with the first paddle
        // Add a bounce angle based on where the ball hits the paddle
        if ball_rect.intersect(*paddle_1).is_some() {
            self.dir.x = self.dir.x.abs();
            let hit_pos = (self.circle.y - paddle_1.y) / paddle_1.h - 0.5;
            self.dir.y += hit_pos * PADDLE_BOUNCE_ANGLE_FACTOR;
            self.dir *= BALL_SPEED_MULTIPLIER;
            self.circle.x = paddle_1.x + paddle_1.w + BALL_RADIUS;
            play_collision_sound(sound).await;
        }

        // Check for collisions with the second paddle
        // Add a bounce angle based on where the ball hits the paddle
        if ball_rect.intersect(*paddle_2).is_some() {
            self.dir.x = -self.dir.x.abs();
            let hit_pos = (self.circle.y - paddle_2.y) / paddle_2.h - 0.5;
            self.dir.y += hit_pos * PADDLE_BOUNCE_ANGLE_FACTOR;
            self.dir *= BALL_SPEED_MULTIPLIER;
            self.circle.x = paddle_2.x - BALL_RADIUS;
            play_collision_sound(sound).await;
        }
    }

    /// Resets the ball to the center of the screen with a random direction
    pub fn reset(&mut self) {
        self.circle = Circle::new(screen_width() / 2., screen_height() / 2., BALL_RADIUS);
        self.dir = Self::random_direction();
    }

    /// Draws the ball on the screen
    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, BALL_RADIUS, BALL_COLOR);
    }
}
