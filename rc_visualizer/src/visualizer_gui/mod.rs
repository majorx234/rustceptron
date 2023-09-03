use eframe::egui;
use eframe::egui::{Color32, TextureHandle};
use egui::Ui;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rc_data::Layer;
use rc_dataset_generator::{layer_fill_circle, layer_fill_rect};

struct Visualizer {
    pub tex_mngr: TextureManager,
    pub texture_id: Option<(egui::Vec2, egui::TextureId)>,
}

impl Default for Visualizer {
    fn default() -> Self {
        Self {
            tex_mngr: TextureManager(vec![Color32::from_rgb(255, 255, 255); 512 * 512], None),
            texture_id: None,
        }
    }
}

impl Visualizer {
    fn ui(&mut self, ui: &mut Ui, layer_data: Option<&Layer<f32>>) {
        if let Some(layer_data) = layer_data {
            self.set_values(ui.ctx(), layer_data);
        }

        if let Some((size, texture_id)) = self.texture_id {
            ui.add(egui::Image::new(texture_id, size));
            ui.ctx().request_repaint();
        }
    }

    fn set_values(&mut self, ctx: &egui::Context, new_layler: &Layer<f32>) {
        self.tex_mngr.update_texture(ctx, new_layler, 512, 512);
        if let Some(ref texture) = self.tex_mngr.1 {
            self.texture_id = Some((egui::Vec2::new(512.0, 512.0), texture.into()));
        }
    }
}

struct TextureManager(Vec<egui::epaint::Color32>, Option<TextureHandle>);

impl TextureManager {
    pub fn update_texture(
        &mut self,
        ctx: &egui::Context,
        layer_data: &Layer<f32>,
        width: usize,
        height: usize,
    ) {
        //ToDo:
        // newimage to self.0 - flatten Vec with Color32 values

        // maybe return an option
        // or handle if pixels.len() < width*height
        let mut pixels: Vec<egui::epaint::Color32> =
            vec![egui::epaint::Color32::from_gray(0); width * height];
        for (pixel, sample) in pixels.iter_mut().zip(layer_data.data.iter()) {
            *pixel = egui::epaint::Color32::from_gray((255.0 * sample) as u8);
        }
        self.1 = Some(ctx.load_texture(
            "color_test_gradient",
            egui::ColorImage {
                size: [width, height],
                pixels,
            },
        ));
    }
}

pub struct VisualizerGui {
    visualizer: Visualizer,
    create_new_image: bool,
    layer_input: Layer<f32>,
    rng: ThreadRng,
    is_initial_image: bool,
}

impl VisualizerGui {
    pub fn new() -> Self {
        Self {
            visualizer: Visualizer::default(),
            create_new_image: false,
            layer_input: Layer::<f32>::new(512, 512),
            rng: thread_rng(),
            is_initial_image: true,
        }
    }
}
impl Default for VisualizerGui {
    fn default() -> Self {
        Self {
            visualizer: Visualizer::default(),
            create_new_image: false,
            layer_input: Layer::<f32>::new(512, 512),
            rng: thread_rng(),
            is_initial_image: true,
        }
    }
}

impl eframe::App for VisualizerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let mut new_image = None;
                if self.is_initial_image {
                    new_image = Some(&self.layer_input);
                    self.is_initial_image = false;
                }
                if self.create_new_image {
                    self.layer_input.clear();
                    let cx: isize = self.rng.gen_range(0..512);
                    let cy: isize = self.rng.gen_range(0..512);
                    let cr: isize = self.rng.gen_range(1..256);
                    layer_fill_circle(&mut self.layer_input, cx, cy, cr, 1.0);
                    new_image = Some(&self.layer_input);
                    self.create_new_image = false;
                }
                self.visualizer.ui(ui, new_image);
                ui.horizontal(|ui| {
                    if ui.button("next").clicked() {
                        self.create_new_image = true;
                    }
                });
            });
        });
    }
}
