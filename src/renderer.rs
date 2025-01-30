use crate::camera::Camera;
use crate::matrices;
use crate::model::ModelPosition;
use crate::model::ModelRotation;
use crate::texture::Texture;
use gl;

#[derive(PartialEq, Copy, Clone)]
pub enum RenderMode {
    Vertex,
    Face,
    Texture,
}

pub struct Renderer {
    shader_program: gl::types::GLuint,
    vao: gl::types::GLuint,
    model_loc: i32,
    pos_offset_loc: i32,
    model_center_loc: i32,
    model_position_loc: i32,
    num_indices: i32,
    texture: Option<Texture>,
    texture_blend_loc: i32,
    texture_sampler_loc: i32,
    texture_blend: f32,
    render_mode: RenderMode,
	current_mode: RenderMode,
    target_mode: RenderMode,
    transition_progress: f32,
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

impl Renderer {
    pub fn new(
        shader_program: gl::types::GLuint,
        vao: gl::types::GLuint,
        model_loc: i32,
        num_indices: i32,
    ) -> Self {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Viewport(0, 0, 1024, 768);
        }

        let pos_offset_loc = unsafe {
            gl::GetUniformLocation(
                shader_program,
                std::ffi::CString::new("posOffset").unwrap().as_ptr(),
            )
        };
        let model_center_loc = unsafe {
            gl::GetUniformLocation(
                shader_program,
                std::ffi::CString::new("modelCenter").unwrap().as_ptr(),
            )
        };
        let model_position_loc = unsafe {
            gl::GetUniformLocation(
                shader_program,
                std::ffi::CString::new("modelPosition").unwrap().as_ptr(),
            )
        };

        let texture_blend_loc = unsafe {
            gl::GetUniformLocation(
                shader_program,
                std::ffi::CString::new("textureBlend").unwrap().as_ptr(),
            )
        };
        let texture_sampler_loc = unsafe {
            gl::GetUniformLocation(
                shader_program,
                std::ffi::CString::new("textureSampler").unwrap().as_ptr(),
            )
        };

        Renderer {
            shader_program,
            vao,
            model_loc,
            pos_offset_loc,
            model_center_loc,
            model_position_loc,
            num_indices,
            texture: None,
            texture_blend_loc,
            texture_sampler_loc,
            texture_blend: 0.0,
            render_mode: RenderMode::Vertex,
			current_mode: RenderMode::Vertex,
			target_mode: RenderMode::Vertex,
			transition_progress: 0.0,
        }
    }

    pub fn render(
        &self,
        model_rotation: &ModelRotation,
        model_center: [f32; 3],
        camera: &Camera,
        model_position: &ModelPosition,
    ) {
        unsafe {
            gl::ClearColor(0.10, 0.10, 0.10, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);

            // Only rotation in model matrix
            let model = matrices::mat4_mul(
                matrices::mat4_rotation_y(model_rotation.y_angle),
                matrices::mat4_rotation_x(model_rotation.x_angle),
            );

            gl::UniformMatrix4fv(self.model_loc, 1, gl::FALSE, model.as_ptr() as *const f32);

            // Handle texture if available
            if let Some(ref texture) = self.texture {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);

                if self.texture_sampler_loc >= 0 {
                    gl::Uniform1i(self.texture_sampler_loc, 0);
                }

                if self.texture_blend_loc >= 0 {
                    gl::Uniform1f(self.texture_blend_loc, self.texture_blend);
                }
            }

            match self.current_mode {
                RenderMode::Vertex => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
                RenderMode::Face | RenderMode::Texture => {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)
                }
            }

            // Set uniforms
            if self.pos_offset_loc >= 0 {
                gl::Uniform1f(self.pos_offset_loc, camera.position[2] - model_position.z);
            }

            if self.model_center_loc >= 0 {
                gl::Uniform3f(
                    self.model_center_loc,
                    model_center[0],
                    model_center[1],
                    model_center[2],
                );
            }

            if self.model_position_loc >= 0 {
                gl::Uniform3f(
                    self.model_position_loc,
                    model_position.x,
                    model_position.y,
                    0.0,
                );
            }

            gl::DrawElements(
                gl::TRIANGLES,
                self.num_indices,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::BindVertexArray(0);
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Result<(), String> {
        self.texture = Some(Texture::new(path)?);
        Ok(())
    }

    pub fn cycle_render_mode(&mut self) {
        self.target_mode = match self.current_mode {
            RenderMode::Vertex => RenderMode::Face,
            RenderMode::Face => RenderMode::Texture,
            RenderMode::Texture => RenderMode::Vertex,
        };
		self.render_mode = self.target_mode;
        self.transition_progress = 0.0;
    }

	pub fn update(&mut self, delta_time: f32) {
        let transition_speed = 3.0;
        self.transition_progress += delta_time * transition_speed;
        self.transition_progress = self.transition_progress.min(1.0);

        self.texture_blend = match (self.current_mode, self.target_mode) {
            (RenderMode::Vertex, RenderMode::Face) => lerp(0.0, 0.5, self.transition_progress),
            (RenderMode::Face, RenderMode::Texture) => lerp(0.5, 1.0, self.transition_progress),
            (RenderMode::Texture, RenderMode::Vertex) => lerp(1.0, 2.0, self.transition_progress),
            _ => self.texture_blend,
        };

        if self.transition_progress >= 1.0 {
            self.current_mode = self.target_mode;
        }
    }
}
