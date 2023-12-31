use rc_data::layer::{activation_fct::ReLU, Layer};
use rc_dataset_generator::layer_fill_rect;

fn main() {
    let mut data_layer: Layer<f32> = Layer::new(50, 50, Box::new(ReLU::new(1.0)));
    layer_fill_rect(&mut data_layer, 30, 30, 15, 15, 1.0);
    data_layer.print();
}
