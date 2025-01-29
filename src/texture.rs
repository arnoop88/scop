use gl;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Texture {
    pub id: gl::types::GLuint,
}

impl Texture {
    pub fn new(path: &str) -> Result<Self, String> {
        let mut file = File::open(Path::new(path)).map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;

        if bytes.len() < 54 {
            // BMP header size
            return Err("Invalid BMP file".to_string());
        }

        // Verify BMP signature
        if bytes[0] != b'B' || bytes[1] != b'M' {
            return Err("Not a valid BMP file".to_string());
        }

        let width = i32::from_le_bytes(bytes[18..22].try_into().unwrap());
        let height = i32::from_le_bytes(bytes[22..26].try_into().unwrap());
        let data_offset = u32::from_le_bytes(bytes[10..14].try_into().unwrap()) as usize;
        let _bpp = u16::from_le_bytes(bytes[28..30].try_into().unwrap()) as usize;

        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::BGR,
                gl::UNSIGNED_BYTE,
                bytes[data_offset..].as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture { id: texture_id })
    }
}
