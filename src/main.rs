use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;

mod ball;
mod constants;
mod egui_menu;
mod paddle;

use crate::ball::Ball;
use crate::constants::*;
use crate::egui_menu::init_egui;
use crate::paddle::Paddle;

/// The main function
#[macroquad::main(conf)]
async fn main() {
    // Load the font
    let font: Font = load_ttf_font("./res/PixelifySans-Regular.ttf")
        .await
        .unwrap();

    // Load the background music
    let bg_music: Sound = load_sound("./assets/sounds/bg_music.wav").await.unwrap();

    let collision_sound: Sound = load_sound("./assets/sounds/pop3.ogg").await.unwrap();

    // Set target fps
    let target_frame_time: f32 = 1. / 60.;

    // Get the time it takes to render a frame
    let frame_time = get_frame_time();

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

    // Play the background music
    play_sound(
        bg_music,
        PlaySoundParams {
            looped: true,
            volume: 0.1,
        },
    );

    // Main game loop
    loop {
        // Initialize egui debug menu
        init_egui(&mut ball);

        // Update game logic
        paddle_1.movement(KeyCode::W, KeyCode::S);
        paddle_2.movement(KeyCode::Up, KeyCode::Down);
        ball.movement();
        ball.collision_with_paddles(&paddle_1.rect, &paddle_2.rect, collision_sound);

        // Check if the ball goes out of the screen (horizontally) and reset ball
        if ball.circle.x + BALL_RADIUS <= 0. {
            ball.reset();
            scores.1 += 1;
        } else if ball.circle.x + BALL_RADIUS >= screen_width() {
            ball.reset();
            scores.0 += 1;
        }

        // If either player has won the game, break the loop and exit the game
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

        // Draw egui debug menu
        egui_macroquad::draw();

        next_frame().await; // Draw the next frame

        // Post process (delay the game to make it run at the target fps)
        if frame_time < target_frame_time {
            let delay_duration = (target_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(delay_duration as u64));
        }
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
            font: *font,
            ..Default::default()
        },
    );
    draw_text_ex(
        format!("{}", scores.1).as_str(),
        screen_width() - 100. - 50.,
        100.,
        TextParams {
            font_size: 70,
            font: *font,
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

/// Checks if either player has won the game
///
/// # Arguments
///
/// * `scores` - The scores to check (tuple of u8)
///
/// # Returns
///
/// * `bool` - Whether either player has won the game
fn win_condition(scores: &(u8, u8)) -> bool {
    scores.0 >= 3 || scores.1 >= 3
}
