use crate::parser::ObjData;
use gl;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub fn setup_opengl_buffers(obj_data: &ObjData) -> (gl::types::GLuint, gl::types::GLuint) {
    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;

    // Create flat index array from faces
    let indices: Vec<u32> = obj_data
        .faces
        .iter()
        .flat_map(|face| face.vertex_indices.iter().cloned())
        .collect();

    // Interleave vertex data
    let mut interleaved_data = Vec::new();
    for (i, vertex) in obj_data.vertices.iter().enumerate() {
        // Position
        interleaved_data.extend_from_slice(vertex);

        // Texture coordinates (with default fallback)
        let tex_coord = obj_data.tex_coords.get(i).unwrap_or(&[0.0, 0.0]);
        interleaved_data.extend_from_slice(tex_coord);

        // Normals (with default fallback)
        let normal = obj_data.normals.get(i).unwrap_or(&[0.0, 1.0, 0.0]);
        interleaved_data.extend_from_slice(normal);
    }

    unsafe {
        // Generate and bind VAO
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Generate and bind VBO
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (interleaved_data.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            interleaved_data.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Generate and bind EBO
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Each vertex has: 3 floats (position) + 2 floats (tex) + 3 floats (normal) = 8 floats total
        let stride = 8 * mem::size_of::<f32>();

        // Position attribute (3 floats)
        gl::VertexAttribPointer(
            0,                          // location
            3,                          // size (3 components)
            gl::FLOAT,                  // type
            gl::FALSE,                  // normalized?
            stride as gl::types::GLint, // stride
            ptr::null(),                // offset
        );
        gl::EnableVertexAttribArray(0);

        // Texture coordinate attribute (2 floats)
        gl::VertexAttribPointer(
            1,                                            // location
            2,                                            // size (2 components)
            gl::FLOAT,                                    // type
            gl::FALSE,                                    // normalized?
            stride as gl::types::GLint,                   // stride
            (3 * mem::size_of::<f32>()) as *const c_void, // offset
        );
        gl::EnableVertexAttribArray(1);

        // Normal attribute (3 floats)
        gl::VertexAttribPointer(
            2,                                            // location
            3,                                            // size (3 components)
            gl::FLOAT,                                    // type
            gl::FALSE,                                    // normalized?
            stride as gl::types::GLint,                   // stride
            (5 * mem::size_of::<f32>()) as *const c_void, // offset
        );
        gl::EnableVertexAttribArray(2);

        // Don't unbind the EBO while a VAO is bound
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    (vao, ebo)
}
