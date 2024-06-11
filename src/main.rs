use macroquad::prelude::*;

mod ball;
mod constants;
mod paddle;

use crate::ball::Ball;
use crate::constants::*;
use crate::paddle::Paddle;

/// The main function
#[macroquad::main(conf)]
async fn main() {
    // Load the font
    let font: Font = load_ttf_font("./res/Roboto-Black.ttf").await.unwrap();

    // Initialize the scores
    let mut scores: (u8, u8) = (0, 0);

    // Object initialization
    let mut paddle_1: Paddle = Paddle::new(Rect::new(
        10.,
        screen_height() / 2. - PADDLE_H / 2.,
        PADDLE_W,
        PADDLE_H,
    ));

    let mut paddle_2: Paddle = Paddle::new(Rect::new(
        screen_width() - 10. - PADDLE_W,
        screen_height() / 2. - PADDLE_H / 2.,
        PADDLE_W,
        PADDLE_H,
    ));

    let mut ball: Ball = Ball::new(Circle::new(
        screen_width() / 2.,
        screen_height() / 2.,
        BALL_RADIUS,
    ));

    // Main game loop
    loop {
        // Update game logic
        paddle_1.movement(KeyCode::W, KeyCode::S);
        paddle_2.movement(KeyCode::Up, KeyCode::Down);
        ball.movement();
        ball.collision_with_paddles(&paddle_1.rect, &paddle_2.rect);

        // Check if the ball goes out of the screen (horizontal) and reset ball
        if ball.circle.x + BALL_RADIUS <= 0. {
            ball.reset();
            scores.1 += 1;
        } else if ball.circle.x + BALL_RADIUS >= screen_width() {
            ball.reset();
            scores.0 += 1;
        }

        if win_condition(&scores) {
            break;
        }

        // Begin drawing
        clear_background(BG_COLOR);
        paddle_1.draw();
        paddle_2.draw();
        ball.draw();
        draw_scores(&font, &scores);
        draw_field();

        next_frame().await;
    }
}

/// Configures the game
fn conf() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

/// Draws the scores on the screen
///
/// # Arguments
///
/// * `font` - The font to use to draw the scores
/// * `scores` - The scores to draw
fn draw_scores(font: &Font, scores: &(u8, u8)) {
    draw_text_ex(
        format!("{}", scores.0).as_str(),
        100.,
        100.,
        TextParams {
            font_size: 70,
            font: Some(font),
            ..Default::default()
        },
    );
    draw_text_ex(
        format!("{}", scores.1).as_str(),
        screen_width() - 100. - 50.,
        100.,
        TextParams {
            font_size: 70,
            font: Some(font),
            ..Default::default()
        },
    );
}

/// Draws the field on the screen
fn draw_field() {
    draw_rectangle_lines(
        0.5,
        0.5,
        screen_width() - 0.5,
        screen_height() - 0.5,
        2.,
        WHITE,
    );
    draw_line(
        screen_width() / 2.,
        0.,
        screen_width() / 2.,
        screen_height(),
        1.,
        WHITE,
    );
    draw_circle_lines(screen_width() / 2., screen_height() / 2., 100., 1., WHITE);
}

fn win_condition(scores: &(u8, u8)) -> bool {
    scores.0 >= 10 || scores.1 >= 10
}
