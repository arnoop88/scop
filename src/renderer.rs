use gl;
use crate::model::{ModelRotation};
use crate::matrices;
use crate::camera::Camera;

pub struct Renderer {
    shader_program: gl::types::GLuint,
    vao: gl::types::GLuint,
    model_loc: i32,
    pos_offset_loc: i32,
    num_indices: i32,
}

impl Renderer {
    pub fn new(shader_program: gl::types::GLuint, vao: gl::types::GLuint, model_loc: i32, num_indices: i32) -> Self {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Viewport(0, 0, 1024, 768);
        }

        let pos_offset_loc = unsafe {
            gl::GetUniformLocation(shader_program, std::ffi::CString::new("posOffset").unwrap().as_ptr())
        };
        
        Renderer {
            shader_program,
            vao,
            model_loc,
            pos_offset_loc,
            num_indices,
        }
    }

    pub fn render(&self, model_rotation: &ModelRotation, model_center: [f32; 3], camera: &Camera) {
        unsafe {
            gl::ClearColor(0.20, 0.20, 0.20, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);

            let model = matrices::mat4_mul(
                matrices::mat4_mul(
                    matrices::mat4_mul(
                        matrices::mat4_translation(-model_center[0], -model_center[1], -model_center[2]),
                        matrices::mat4_rotation_y(model_rotation.y_angle)
                    ),
                    matrices::mat4_rotation_x(model_rotation.x_angle)
                ),
                matrices::mat4_scale(0.3, 0.3, 0.3)
            );

            gl::UniformMatrix4fv(self.model_loc, 1, gl::FALSE, model.as_ptr() as *const f32);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            if self.pos_offset_loc >= 0 {
                gl::Uniform1f(self.pos_offset_loc, camera.position[2]);
            }

            gl::DrawElements(
                gl::TRIANGLES,
                self.num_indices,
                gl::UNSIGNED_INT,
                std::ptr::null()
            );

            gl::BindVertexArray(0);
        }
    }
} 