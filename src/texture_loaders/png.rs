use std::fs::File;
use std::io::Read;

use crate::gl_wrapper::texture_object::Texture;
use crate::gl_wrapper::safe::safe_gl as sgl;

impl Texture {
    pub fn from_png(data: &[u8]) -> Texture {
        let mut texture_object = Texture::new();
        texture_object.bind();
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
        sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());
        let bitmap_result = imagine::png::png_try_bitmap_rgba(&data, true);
        texture_object.load_bitmap(&bitmap_result.unwrap()).unwrap();
        texture_object
    }

    pub fn from_png_file(mut file: File) -> Texture {
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        return Texture::from_png(&buffer);
    }
}