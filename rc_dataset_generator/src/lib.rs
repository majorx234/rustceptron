use rc_data::Layer;

#[inline(always)]
pub fn clampi(x: isize, low: isize, high: isize) -> isize {
    if x < low {
        return low;
    };
    if x > high {
        return high;
    };
    x
}

pub fn layer_fill_rect(
    layer: &mut Layer<f32>,
    cx: isize,
    cy: isize,
    w: isize,
    h: isize,
    value: f32,
) {
    assert!(w > 0);
    assert!(h > 0);
    let x0 = clampi(cx - w / 2, 0, layer.width as isize - 1);
    let y0 = clampi(cy - h / 2, 0, layer.height as isize - 1);
    let x1 = clampi(cx + w / 2, 0, layer.width as isize - 1);
    let y1 = clampi(cy + h / 2, 0, layer.height as isize - 1);
    for y in y0..y1 {
        for x in x0..x1 {
            layer.data[y as usize * (layer.width - 1) + x as usize] = value;
        }
    }
}

pub fn layer_fill_circle(layer: &mut Layer<f32>, cx: isize, cy: isize, r: isize, value: f32) {
    assert!(r > 0);
    let x0 = clampi(cx - r, 0, layer.width as isize - 1);
    let y0 = clampi(cy - r, 0, layer.height as isize - 1);
    let x1 = clampi(cx + r, 0, layer.width as isize - 1);
    let y1 = clampi(cy + r, 0, layer.height as isize - 1);
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= r * r {
                layer.data[y as usize * (layer.width - 1) + x as usize] = value;
            }
        }
    }
}
