use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use gl;
use imagine::Bitmap;
use super::safe::safe_gl as sgl;

use super::check_gl_errors::check_gl_errors;

#[derive(Debug)]
pub struct Texture(u32);

pub enum TextureWrapping {
    Repeat = gl::REPEAT as isize,
    Mirrored = gl::MIRRORED_REPEAT as isize,
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    ClampToBorder = gl::CLAMP_TO_BORDER as isize
}

impl Texture {
    pub fn new() -> Self {
        let mut val = 0;
        unsafe { gl::GenTextures(1, &mut val) }
        check_gl_errors();
        Texture(val)
    }
    
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.0) }
        check_gl_errors()
    }

    pub fn load_bitmap(&mut self, bitmap: &Bitmap) -> Result<(), Error> {
        sgl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGBA.try_into().unwrap(), 
            bitmap.width.try_into().unwrap(), 
            bitmap.height.try_into().unwrap(), 
            0, 
            gl::RGBA.try_into().unwrap(), 
            gl::UNSIGNED_BYTE, 
            bytemuck::cast_slice(bitmap.pixels.as_slice())
        );
        Ok(())
    }

    pub fn from_file(mut file: File) -> Result<Self, Error> {
        let mut texture_object = Self::new();
        let mut buffer = vec![];
        texture_object.bind();
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());
        file.read_to_end(&mut buffer)?;
        let bitmap_result = imagine::png::png_try_bitmap_rgba(&buffer, true);
        if bitmap_result.is_err() {
            return Err(std::io::Error::from(ErrorKind::InvalidData));
        }
        texture_object.load_bitmap(&bitmap_result.unwrap())?;
        Ok(texture_object)
    }
}