pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
}

impl Camera {
    pub fn new(_width: u32, _height: u32) -> Self {
        Camera {
            position: [0.0, 0.0, 10.0],
            target: [0.0, 0.0, 0.0],
        }
    }

    pub fn update_target(&mut self, model_center: [f32; 3]) {
        self.target = model_center;
    }

    pub fn zoom(&mut self, delta: f32) {
        const MIN_DISTANCE: f32 = 0.0;

        let new_z = self.position[2] + delta;
        self.position[2] = new_z.max(MIN_DISTANCE);
    }
}
