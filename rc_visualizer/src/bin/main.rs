mod visualizer_gui;
use crate::visualizer_gui::Visualizer;

fn main() {
    let mut visualizer_app = VisualizerGui::new();
    let mut options = eframe::NativeOptions::default();
    let window_size: eframe::egui::Vec2 = eframe::egui::Vec2::new(400.0, 400.0);
    options.initial_window_size = Some(window_size);

    eframe::run_native(
        "RcVisualizerGui",
        options,
        Box::new(|_cc| Box::new(visualizer_app)),
    );
}
