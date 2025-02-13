extern crate gl;
extern crate sdl2;

mod camera;
mod matrices;
mod model;
mod opengl_setup;
mod parser;
mod renderer;
mod shaders;
mod texture;

use camera::Camera;
use renderer::Renderer;
use sdl2::keyboard::Keycode;	
use sdl2::event::WindowEvent;
use std::ffi::CString;
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // argument handler
    let (model_path, texture_path) = match args.len() {
        1 => ("models/42.obj", "textures/sigma_cat.bmp"),
        2 => {
            let arg = args[1].as_str();
            if arg.ends_with(".obj") {
                (arg, "textures/sigma_cat.bmp")
            } else if arg.ends_with(".bmp") {
                ("models/42.obj", arg)
            } else {
				eprintln!("Invalid arguments\nModels accepted: .obj, Textures accepted: .bmp");
				std::process::exit(1);
			}
        },
        3 => {
            let (arg1, arg2) = (args[1].as_str(), args[2].as_str());
            if arg1.ends_with(".obj") && arg2.ends_with(".bmp") {
                (arg1, arg2)
            } else if arg1.ends_with(".bmp") && arg2.ends_with(".obj") {
                (arg2, arg1)
            } else {
				eprintln!("Invalid arguments\nModels accepted: .obj, Textures accepted: .bmp");
				std::process::exit(1);
			}
        },
        _ => {
            eprintln!("Error: Too many arguments");
            std::process::exit(1);
        }
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let mut window_width = 1024 as i32;
	let mut window_height = 768 as i32;
	let mut minimized = false;
    let window = video_subsystem
        .window("SCOP", window_width as u32, window_height as u32)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| window.subsystem().gl_get_proc_address(s) as *const _);

    let model_data = parser::ObjData::parse(model_path).unwrap_or_else(|_| {
        eprintln!("Failed to load model: {}", model_path);
        std::process::exit(1);
    });
    
    let model_center = model::calculate_model_center(&model_data.vertices);
    let (vao, _ebo) = opengl_setup::setup_opengl_buffers(&model_data);

    let vertex_shader =
        shaders::compile_shader(include_str!("shaders/vertex.glsl"), gl::VERTEX_SHADER).unwrap();
    let fragment_shader =
        shaders::compile_shader(include_str!("shaders/fragment.glsl"), gl::FRAGMENT_SHADER)
            .unwrap();
    let shader_program = shaders::link_program(vertex_shader, fragment_shader).unwrap();

    let model_loc =
        unsafe { gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr()) };

    let mut camera = Camera::new(window_width, window_height);
    camera.update_target(model_center);
    let mut model_rotation = model::ModelRotation::new();
    let mut model_position = model::ModelPosition::new();
    let mut last_frame = Instant::now();
    let mut renderer = Renderer::new(
        shader_program,
        vao,
        model_loc,
        (model_data.faces.len() * 3) as i32,
    );

    renderer.load_texture(texture_path).unwrap_or_else(|_| {
        eprintln!("Failed to load texture: {}", texture_path);
        std::process::exit(1);
    });

    'mainloop: loop {
        let current_frame = Instant::now();
        let delta_time = (current_frame - last_frame).as_secs_f32();
        last_frame = current_frame;

		renderer.update(delta_time);

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                sdl2::event::Event::Window { win_event, .. } => match win_event {
					WindowEvent::Resized(width, height) |
					WindowEvent::SizeChanged(width, height) => {
						window_width = width as i32;
						window_height = height as i32;
					}
					WindowEvent::Minimized => minimized = true,
					WindowEvent::Restored => minimized = false,
					_ => {}
				},
				sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape | Keycode::Q => break 'mainloop,
                    Keycode::A => model_rotation.y_angle -= 0.1,
                    Keycode::D => model_rotation.y_angle += 0.1,
                    Keycode::W => model_rotation.x_angle -= 0.1,
                    Keycode::S => model_rotation.x_angle += 0.1,
                    Keycode::Left => model_position.x -= 0.1,
                    Keycode::Right => model_position.x += 0.1,
                    Keycode::Up => model_position.y += 0.1,
                    Keycode::Down => model_position.y -= 0.1,
                    Keycode::Z => camera.zoom(-0.2),
                    Keycode::X => camera.zoom(0.2),
                    Keycode::E => renderer.cycle_render_mode(),
                    _ => {}
                },
                _ => {}
            }
        }
		if !minimized {
			unsafe {
				gl::Viewport(0, 0, window_width, window_height);
			}
			
			if window_width != camera.width || window_height != camera.height {
				camera.update_resolution(window_width, window_height);
			}

			renderer.render(&model_rotation, model_center, &camera, &model_position);
			window.gl_swap_window();
		}
    }
}
