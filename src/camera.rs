pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub width: i32,
    pub height: i32,
    aspect_ratio: f32,
    pub projection: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        Camera {
            position: [0.0, 0.0, 10.0],
            target: [0.0, 0.0, 0.0],
            width,
            height,
            aspect_ratio,
            projection: Camera::calculate_projection(aspect_ratio),
        }
    }

    pub fn update_resolution(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.aspect_ratio = width as f32 / height as f32;
        self.projection = Camera::calculate_projection(self.aspect_ratio);
    }

    fn calculate_projection(aspect_ratio: f32) -> [[f32; 4]; 4] {
        let fov = 60.0f32.to_radians();
        let near = 0.1;
        let far = 1000.0;
        
        let scale = (fov / 2.0).tan();
        let r = aspect_ratio * scale;
        let l = -r;
        let t = scale;
        let b = -t;

        // Same perspective matrix calculation as in your vertex shader
        [
            [2.0 / (r - l), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (t - b), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near)],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    // Keep existing methods
    pub fn update_target(&mut self, model_center: [f32; 3]) {
        self.target = model_center;
    }

    pub fn zoom(&mut self, delta: f32) {
        let new_z = self.position[2] + delta;
        self.position[2] = new_z.max(0.0); // minimum distance
    }
}