use crate::Ball;

/// Initialize egui debug menu
///
/// # Arguments
///
/// * `ball` - A mutable reference to the `Ball` struct
pub fn init_egui(ball: &mut Ball) {
    egui_macroquad::ui(|ctx: &egui_macroquad::egui::Context| {
        egui_macroquad::egui::Window::new("Developer Tools").show(ctx, |ui| {
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
