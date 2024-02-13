use gl;
use gl::types::*;

use super::check_gl_errors::check_gl_errors;

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader(pub GLuint);
impl Shader {
    pub fn new(ty: ShaderType) -> Option<Self> {
      let shader = unsafe { 
        let ans = gl::CreateShader(ty as GLenum);
        check_gl_errors();
        ans
    };
      if shader != 0 {
        Some(Self(shader))
      } else {
        None
      }
    }
  
    pub fn set_source(&self, src: &str) {
      unsafe {
        gl::ShaderSource(
          self.0,
          1,
          &(src.as_bytes().as_ptr().cast()),
          &(src.len().try_into().unwrap()),
        );
      }
    }
  
    pub fn compile(&self) {
      unsafe { gl::CompileShader(self.0) };
    }
  
    pub fn compile_success(&self) -> bool {
      let mut compiled = 0;
      unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
      compiled == i32::from(gl::TRUE)
    }
  
    pub fn info_log(&self) -> String {
      let mut needed_len = 0;
      unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
      let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
      let mut len_written = 0_i32;
      unsafe {
        gl::GetShaderInfoLog(
          self.0,
          v.capacity().try_into().unwrap(),
          &mut len_written,
          v.as_mut_ptr().cast(),
        );
        v.set_len(len_written.try_into().unwrap());
      }
      String::from_utf8_lossy(&v).into_owned()
    }
  
    pub fn delete(self) {
        unsafe { 
            gl::DeleteShader(self.0);
            check_gl_errors();
        };
    }
  
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
      let id = Self::new(ty)
        .ok_or_else(|| "Couldn't allocate new shader".to_string())?;
      id.set_source(source);
      id.compile();
      if id.compile_success() {
        Ok(id)
      } else {
        let out = id.info_log();
        id.delete();
        Err(out)
      }
    }
}