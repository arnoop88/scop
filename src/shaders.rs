use gl;

pub fn compile_shader(
    src: &str,
    shader_type: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = std::ffi::CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let error = create_whitespace_cstring_with_len(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
            return Err(error.to_string_lossy().into_owned());
        }
    }
    Ok(shader)
}

fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
    // Create an empty CString with space for len characters
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}

pub fn link_program(
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
) -> Result<gl::types::GLuint, String> {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let error = create_whitespace_cstring_with_len(len as usize);
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
            return Err(error.to_string_lossy().into_owned());
        }
    }
    Ok(program)
}
