use crate::egui::Ui;
use eframe::egui;
use eframe::egui::{lerp, Color32, Rgba, TextureHandle};

struct Visualizer {
    pub tex_mngr: TextureManager,
    pub texture_id: Option<(egui::Vec2, egui::TextureId)>,
}

impl Default for Visualizer {
    fn default() -> Self {
        Self {
            tex_mngr: TextureManager(vec![Color32::from_rgb(255, 255, 255); 1024 * 512], None),
            texture_id: None,
        }
    }
}

impl Visualizer {
    fn ui(&mut self, ui: &mut Ui, image_data: Vec<Vec<f32>>) {
        self.set_values(ui.ctx(), image_data);

        if let Some((size, texture_id)) = self.texture_id {
            ui.add(egui::Image::new(texture_id, size));
            ui.ctx().request_repaint();
        }
    }

    fn set_values(&mut self, ctx: &egui::Context, new_image: Vec<Vec<f32>>) {
        let new_image_int: Vec<Vec<u8>> = Vec::new();
        for pixel_row in new_image.iter() {
            let pixel_row_int = pixel_row
                .iter()
                .map(|&value| (255.0 * value.abs()) as u8)
                .collect();
            new_image_int.push(pixel_row_int);
        }
        self.tex_mngr.update_texture(ctx, new_image_int, 512, 512);
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
        new_image: Vec<Vec<u8>>,
        width: usize,
        height: usize,
    ) {
        //ToDo:
        // newimage to self.0 - flatten Vec with Color32 values

        // maybe return an option
        // or handle if pixels.len() < width*height
        let pixels: Vec<egui::epaint::Color32> = self.0.clone();
        self.1 = Some(ctx.load_texture(
            "color_test_gradient",
            egui::ColorImage {
                size: [width, height],
                pixels,
            },
        ));
    }
}
