pub struct ModelRotation {
    pub x_angle: f32,
    pub y_angle: f32,
}

impl ModelRotation {
    pub fn new() -> Self {
        ModelRotation {
            x_angle: 0.0,
            y_angle: 0.0,
        }
    }
}

pub fn calculate_model_center(vertices: &[[f32; 3]]) -> [f32; 3] {
    // First find the bounding box
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];
	
	[
        (min[0] + max[0]) / 2.0,
        (min[1] + max[1]) / 2.0,
        (min[2] + max[2]) / 2.0,
    ]
}

