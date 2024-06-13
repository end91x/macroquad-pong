use crate::Ball;

/// Initialize egui debug menu
///
/// # Arguments
///
/// * `ball` - A mutable reference to the `Ball` struct
pub fn init_egui(ball: &mut Ball) {
    egui_macroquad::ui(|ctx: &egui_macroquad::egui::Context| {
        let screen_width = macroquad::prelude::screen_width();

        // Calculate the position to center the window at the top of the screen
        let window_width = 400.0; // Define a suitable width for your window
        let window_pos_x = (screen_width - window_width / 2.0) / 2.0; // Center horizontally
        let window_pos_y = 5.0; // Small offset from the top of the screen

        egui_macroquad::egui::Window::new("Developer Tools")
            .fixed_pos(egui_macroquad::egui::pos2(window_pos_x, window_pos_y))
            .fixed_size(egui_macroquad::egui::vec2(window_width, 200.0)) // Set a fixed size for the window
            .show(ctx, |ui| {
                egui_macroquad::egui::CollapsingHeader::new("Ball Physics")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Position");
                            ui.label(format!("x: {:.2} y: {:.2}", ball.circle.x, ball.circle.y));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Velocity");
                            ui.label(format!("x: {:.2} y: {:.2}", ball.dir.x, ball.dir.y));
                        });
                    });
            });
    });
}
