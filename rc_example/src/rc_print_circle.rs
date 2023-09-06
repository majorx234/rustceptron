use rc_data::layer::Layer;
use rc_dataset_generator::layer_fill_circle;

fn main() {
    let mut data_layer: Layer<f32> = Layer::new(12, 12);
    layer_fill_circle(&mut data_layer, 4, 5, 4, 1.0);
    data_layer.print();
}
