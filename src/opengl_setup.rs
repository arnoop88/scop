use gl;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use crate::parser::ObjData;
use std::cmp::min;

pub fn setup_opengl_buffers(obj_data: &ObjData) -> (gl::types::GLuint, gl::types::GLuint) {
    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;

    // Create flat index array from faces
    let indices: Vec<u32> = obj_data.faces
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

    // Debug prints
    println!("Interleaved data size: {} floats", interleaved_data.len());
    println!("Indices size: {} indices", indices.len());
    println!("First few indices: {:?}", &indices[..9]);
    
    // Verify indices are within bounds
    let max_index = indices.iter().max().unwrap();
    let vertex_count = obj_data.vertices.len();
    println!("Max index: {}, Vertex count: {}", max_index, vertex_count);
    assert!(*max_index < vertex_count as u32, "Index out of bounds!");

    // After creating interleaved data, add this debug print
    println!("First 24 floats of interleaved data:");
    for i in 0..min(24, interleaved_data.len()) {
        print!("{:.3} ", interleaved_data[i]);
        if (i + 1) % 8 == 0 { println!(); }
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
        println!("Stride size in bytes: {}", stride);

        // Position attribute (3 floats)
        gl::VertexAttribPointer(
            0,                          // location
            3,                          // size (3 components)
            gl::FLOAT,                  // type
            gl::FALSE,                  // normalized?
            stride as gl::types::GLint, // stride
            ptr::null()                 // offset
        );
        gl::EnableVertexAttribArray(0);
        
        // Texture coordinate attribute (2 floats)
        gl::VertexAttribPointer(
            1,                          // location
            2,                          // size (2 components)
            gl::FLOAT,                  // type
            gl::FALSE,                  // normalized?
            stride as gl::types::GLint, // stride
            (3 * mem::size_of::<f32>()) as *const c_void // offset
        );
        gl::EnableVertexAttribArray(1);
        
        // Normal attribute (3 floats)
        gl::VertexAttribPointer(
            2,                          // location
            3,                          // size (3 components)
            gl::FLOAT,                  // type
            gl::FALSE,                  // normalized?
            stride as gl::types::GLint, // stride
            (5 * mem::size_of::<f32>()) as *const c_void // offset
        );
        gl::EnableVertexAttribArray(2);

        // Verify our setup
        let mut current_vao = 0;
        let mut current_vbo = 0;
        let mut current_ebo = 0;
        gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
        gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut current_vbo);
        gl::GetIntegerv(gl::ELEMENT_ARRAY_BUFFER_BINDING, &mut current_ebo);
        println!("Current bindings - VAO: {}, VBO: {}, EBO: {}", current_vao, current_vbo, current_ebo);

        // After setting up vertex attributes, verify them
        let mut enabled = 0;
        gl::GetVertexAttribiv(0, gl::VERTEX_ATTRIB_ARRAY_ENABLED, &mut enabled);
        println!("Attribute 0 enabled: {}", enabled);
        
        let mut size = 0;
        gl::GetVertexAttribiv(0, gl::VERTEX_ATTRIB_ARRAY_SIZE, &mut size);
        println!("Attribute 0 size: {}", size);
        
        let mut stride = 0;
        gl::GetVertexAttribiv(0, gl::VERTEX_ATTRIB_ARRAY_STRIDE, &mut stride);
        println!("Attribute 0 stride: {}", stride);

        // Read back the first vertex from the buffer to verify it's correct
        let mut test_data: [f32; 8] = [0.0; 8];
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::GetBufferSubData(
            gl::ARRAY_BUFFER,
            0,
            (8 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            test_data.as_mut_ptr() as *mut c_void
        );
        println!("First vertex data read back from buffer:");
        for i in 0..8 {
            print!("{:.3} ", test_data[i]);
        }
        println!();

        // Don't unbind the EBO while a VAO is bound
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    (vao, ebo)
}
