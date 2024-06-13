use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::constants::BG_COLOR;
use crate::game_state::GameState;

/// Draws the main menu and returns the next state of the game
///
/// # Returns
///
/// * `GameState` - The next state of the game (Game or Exit)
pub async fn draw_main_menu() -> GameState {
    let mut next_state: Option<GameState> = None;

    loop {
        clear_background(BG_COLOR);

        egui_macroquad::ui(|ctx| {
            let screen_width = screen_width();
            let screen_height = screen_height();

            let mut style: egui::Style = (*ctx.style()).clone();
            style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(20.0);
            style.visuals.widgets.inactive.rounding = egui::Rounding::same(20.0);
            style.visuals.widgets.hovered.rounding = egui::Rounding::same(20.0);
            style.visuals.widgets.active.rounding = egui::Rounding::same(20.0);
            style.visuals.window_fill = egui::Color32::from_rgb(13, 13, 36);

            style
                .text_styles
                .get_mut(&egui::TextStyle::Button)
                .unwrap()
                .size = 24.0;

            ctx.set_style(style);

            egui::Window::new("Main Menu")
                .resizable(false)
                .fixed_pos(egui::pos2(0.0, 0.0))
                .min_width(screen_width * 0.5)
                .min_height(screen_height * 0.5)
                .default_size(egui::vec2(screen_width * 0.5, screen_height * 0.5))
                .show(ctx, |ui| {
                    ui.heading("Welcome to the Game!");

                    ui.add_space(20.0);

                    let button_size = egui::vec2(785.0, 100.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Start Game"))
                        .clicked()
                    {
                        next_state = Some(GameState::Game);
                    }

                    ui.add_space(20.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Exit"))
                        .clicked()
                    {
                        next_state = Some(GameState::Exit);
                    }

                    ui.add_space(20.0);
                });
        });

        egui_macroquad::draw();

        next_frame().await;

        if let Some(state) = next_state {
            return state;
        }
    }
}
