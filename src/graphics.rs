use macroquad::prelude::*;

use crate::constants::*;

/// Draws the scores on the screen
///
/// # Arguments
///
/// * `font` - The font to use to draw the scores
/// * `scores` - The scores to draw
pub fn draw_scores(font: &Font, scores: &(u8, u8)) {
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
pub fn draw_field() {
    draw_rectangle_lines(
        0.5,
        0.5,
        screen_width() - 0.5,
        screen_height() - 0.5,
        2.,
        FIELD_COLOR,
    );
    draw_line(
        screen_width() / 2.,
        0.,
        screen_width() / 2.,
        screen_height(),
        1.,
        FIELD_COLOR,
    );
    draw_circle_lines(
        screen_width() / 2.,
        screen_height() / 2.,
        100.,
        1.,
        FIELD_COLOR,
    );
}

/// Draws the wall on the screen
///
/// # Arguments
///
/// * `tile_rect` - The rectangle to use to draw the tiles
/// * `tileset` - The tileset to use to draw the tiles
pub fn draw_wall(tile_rect: Rect, tileset: Texture2D) {
    let num_tiles: i32 = (screen_width() / tile_rect.w).ceil() as i32;

    for i in 0..num_tiles {
        draw_texture_ex(
            tileset,
            i as f32 * tile_rect.w,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(tile_rect.size()),
                source: Some(tile_rect),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    for i in 0..num_tiles {
        draw_texture_ex(
            tileset,
            i as f32 * tile_rect.w,
            screen_height() - tile_rect.h,
            WHITE,
            DrawTextureParams {
                dest_size: Some(tile_rect.size()),
                source: Some(tile_rect),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }
}
