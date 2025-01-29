pub struct ModelRotation {
    pub x_angle: f32,
    pub y_angle: f32,
}

pub struct ModelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ModelRotation {
    pub fn new() -> Self {
        ModelRotation {
            x_angle: 0.0,
            y_angle: 0.0,
        }
    }
}

impl ModelPosition {
    pub fn new() -> Self {
        ModelPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub fn calculate_model_center(vertices: &[[f32; 3]]) -> [f32; 3] {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for vertex in vertices {
        for i in 0..3 {
            min[i] = min[i].min(vertex[i]);
            max[i] = max[i].max(vertex[i]);
        }
    }

    let center = [
        (min[0] + max[0]) / 2.0,
        (min[1] + max[1]) / 2.0,
        min[2] + (max[2] - min[2]) * 0.5,
    ];

    center
}
