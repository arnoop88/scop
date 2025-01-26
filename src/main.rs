extern crate gl;
extern crate sdl2;

mod matrices;
mod parser;
mod shaders;
mod opengl_setup;
mod camera;
mod model;
mod renderer;

use std::ffi::CString;
use std::f32::consts::PI;
use camera::Camera;
use model::{ModelRotation, calculate_model_center};
use renderer::Renderer;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
	
    let window = video_subsystem
        .window("SCOP", 1024, 768)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| window.subsystem().gl_get_proc_address(s) as *const _);

    let model_data = parser::ObjData::parse("models/42.obj").unwrap();
    let model_center = calculate_model_center(&model_data.vertices);
    let (vao, _ebo) = opengl_setup::setup_opengl_buffers(&model_data);
    
    let vertex_shader = shaders::compile_shader(include_str!("shaders/vertex.glsl"), gl::VERTEX_SHADER).unwrap();
    let fragment_shader = shaders::compile_shader(include_str!("shaders/fragment.glsl"), gl::FRAGMENT_SHADER).unwrap();
    let shader_program = shaders::link_program(vertex_shader, fragment_shader).unwrap();

    let model_loc = unsafe { 
        gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr())
    };

    let mut camera = Camera::new(1024, 768);
    let mut model_rotation = ModelRotation::new();
    let renderer = Renderer::new(
        shader_program, 
        vao, 
        model_loc,
        (model_data.faces.len() * 3) as i32
    );

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'mainloop,
                        Keycode::A => model_rotation.y_angle -= 0.1,
                        Keycode::D => model_rotation.y_angle += 0.1,
                        Keycode::W => model_rotation.x_angle -= 0.1,
                        Keycode::S => model_rotation.x_angle += 0.1,
                        Keycode::Up => camera.position[2] = (camera.position[2] - 0.2).max(2.0),
                        Keycode::Down => camera.position[2] = (camera.position[2] + 0.2).min(15.0),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        renderer.render(&model_rotation, model_center, &camera);
        window.gl_swap_window();
    }
}
