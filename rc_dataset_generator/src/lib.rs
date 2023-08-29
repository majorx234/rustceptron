use rc_data::Layer;

#[inline(always)]
fn clampi(x: usize, low: usize, high: usize) -> usize {
    if x < low {
        return low;
    };
    if x > high {
        return high;
    };
    x
}

fn layer_fill_rect(layer: &mut Layer<f32>, x: usize, y: usize, w: usize, h: usize, value: f32) {
    assert!(w > 0);
    assert!(h > 0);
    let x0 = clampi(x, 0, layer.width - 1);
    let y0 = clampi(y, 0, layer.height - 1);
    let x1 = clampi(x0 + w, 0, layer.width - 1);
    let y1 = clampi(y0 + h, 0, layer.height - 1);
    for y in y0..y1 {
        for x in x0..x1 {
            layer.data[y * (layer.width - 1) + x] = value;
        }
    }
}
