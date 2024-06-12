use macroquad::prelude::*;

use crate::constants::*;

/// Represents a paddle
#[derive(Copy, Clone)]
pub struct Paddle {
    /// The rectangle that defines the paddle's position and size
    pub rect: Rect,
}

impl Paddle {
    /// Creates a new paddle with the given rectangle
    ///
    /// # Arguments
    ///
    /// * `rect` - The rectangle that defines the paddle's position and size
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }

    /// Moves the paddle up or down based on the key pressed
    ///
    /// # Arguments
    ///
    /// * `up` - The key code for moving the paddle up
    /// * `down` - The key code for moving the paddle down
    pub fn movement(&mut self, up: KeyCode, down: KeyCode) {
        // Move the paddle up or down based on the key pressed
        if is_key_down(up) {
            self.rect.y -= 1. * PADDLE_SPEED;
        } else if is_key_down(down) {
            self.rect.y += 1. * PADDLE_SPEED;
        }

        // Ensure the paddle stays within the screen
        if self.rect.y <= 0. + 32. {
            self.rect.y = 0. + 32.;
        } else if self.rect.y >= screen_height() - PADDLE_H - 32. {
            self.rect.y = screen_height() - PADDLE_H - 32.;
        }
    }

    /// Draws the paddle on the screen
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            PADDLE_COLOR,
        );
    }
}
