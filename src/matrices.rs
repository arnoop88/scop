pub type Mat4 = [[f32; 4]; 4];

pub fn mat4_mul(a: Mat4, b: Mat4) -> Mat4 {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] =
                a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    result
}

pub fn mat4_rotation_y(angle: f32) -> Mat4 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    [
        [cos_theta, 0.0, sin_theta, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin_theta, 0.0, cos_theta, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn mat4_rotation_x(angle: f32) -> Mat4 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos_theta, -sin_theta, 0.0],
        [0.0, sin_theta, cos_theta, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
