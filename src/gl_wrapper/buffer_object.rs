

use gl::types::*;

use super::check_gl_errors::check_gl_errors;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
  Array = gl::ARRAY_BUFFER as isize,
  ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

#[derive(Debug)]
pub struct Buffer(pub GLuint);
impl Buffer {
  pub fn new() -> Option<Self> {
    let mut vbo = 0;
    unsafe {
      gl::GenBuffers(1, &mut vbo);
      check_gl_errors();
    }
    if vbo != 0 {
      Some(Self(vbo))
    } else {
      None
    }
  }

  pub fn bind(&self, ty: BufferType) {
    unsafe { 
        gl::BindBuffer(ty as GLenum, self.0);
        check_gl_errors();
    }
  }

  pub fn clear_binding(ty: BufferType) {
    unsafe { 
        gl::BindBuffer(ty as GLenum, 0);
        check_gl_errors();
    }
  }
}

pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
      gl::BufferData(
        ty as GLenum,
        data.len().try_into().unwrap(),
        data.as_ptr().cast(),
        usage,
      );
      check_gl_errors();
    }
  }