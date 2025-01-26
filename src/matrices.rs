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

pub fn mat4_identity() -> Mat4 {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn mat4_translation(x: f32, y: f32, z: f32) -> Mat4 {
    [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]
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
        [1.0, 0.0,      0.0,       0.0],
        [0.0, cos_theta, -sin_theta, 0.0],
        [0.0, sin_theta, cos_theta,  0.0],
        [0.0, 0.0,      0.0,        1.0],
    ]
}

pub fn mat4_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov / 2.0).tan();
    [
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far)],
        [0.0, 0.0, -1.0, 0.0],
    ]
}

pub fn look_at(eye: [f32; 3], target: [f32; 3], up: [f32; 3]) -> Mat4 {
    let mut f = [
        target[0] - eye[0],
        target[1] - eye[1],
        target[2] - eye[2],
    ];
    let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt();
    f = [f[0] / len, f[1] / len, f[2] / len];
    
    let mut s = cross(f, up);
    let len = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
    s = [s[0] / len, s[1] / len, s[2] / len];
    
    let u = cross(s, f);
    
    [
        [s[0],     s[1],     s[2],    -dot(s, eye)],
        [u[0],     u[1],     u[2],    -dot(u, eye)],
        [-f[0],    -f[1],    -f[2],    dot(f, eye)],
        [0.0,      0.0,      0.0,      1.0],
    ]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn mat4_scale(x: f32, y: f32, z: f32) -> Mat4 {
    [
        [x,   0.0, 0.0, 0.0],
        [0.0, y,   0.0, 0.0],
        [0.0, 0.0, z,   0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn transform_point(mat: Mat4, point: [f32; 3]) -> [f32; 3] {
    let p = [
        mat[0][0] * point[0] + mat[0][1] * point[1] + mat[0][2] * point[2] + mat[0][3],
        mat[1][0] * point[0] + mat[1][1] * point[1] + mat[1][2] * point[2] + mat[1][3],
        mat[2][0] * point[0] + mat[2][1] * point[1] + mat[2][2] * point[2] + mat[2][3],
    ];
    p
}
