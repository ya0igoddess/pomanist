extern crate gl;

use gl::types::*;

use super::check_gl_errors::check_gl_errors;

#[derive(Debug)]
pub struct ArrayObject(GLuint);

impl ArrayObject {
    pub fn new() -> Option<Self> {
        let mut val = 0u32;
        unsafe {
            gl::CreateVertexArrays(1, &mut val);
            check_gl_errors();
        }
        if val != 0 {
            Some(Self(val))
        } else {
            None
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
            check_gl_errors();
        }
    }
}