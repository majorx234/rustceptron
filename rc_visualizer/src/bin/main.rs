use visualizer::visualizer_gui::VisualizerGui;

fn main() {
    let visualizer_app = VisualizerGui::new();
    let mut options = eframe::NativeOptions::default();
    let window_size: eframe::egui::Vec2 = eframe::egui::Vec2::new(512.0, 542.0);
    options.initial_window_size = Some(window_size);

    eframe::run_native(
        "RcVisualizerGui",
        options,
        Box::new(|_cc| Box::new(visualizer_app)),
    );
}
