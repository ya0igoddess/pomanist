use super::check_gl_errors::check_gl_errors;
use super::safe::safe_gl::*;
use super::shader_object::Shader;
use super::shader_object::ShaderType;
use gl::types::*;
use std::ffi::CStr;

#[derive(Debug)]
pub struct ShaderProgram(pub GLuint);

#[derive(Debug)]
pub struct UniformLocation(pub GLint);

#[derive(Debug)]
pub struct VertexAttrib(pub GLint);

impl ShaderProgram {
  pub fn new() -> Option<Self> {
    let prog = unsafe { gl::CreateProgram() };
    if prog != 0 {
      Some(Self(prog))
    } else {
      None
    }
  }

  pub fn attach_shader(&self, shader: &Shader) {
    unsafe { gl::AttachShader(self.0, shader.0) };
    check_gl_errors();
  }

  pub fn link_program(&self) {
    unsafe { gl::LinkProgram(self.0) };
    check_gl_errors();
  }

  pub fn link_success(&self) -> bool {
    let mut success = 0;
    unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
    success == i32::from(gl::TRUE)
  }

  pub fn info_log(&self) -> String {
    let mut needed_len = 0;
    unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
    let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
    let mut len_written = 0_i32;
    unsafe {
      gl::GetProgramInfoLog(
        self.0,
        v.capacity().try_into().unwrap(),
        &mut len_written,
        v.as_mut_ptr().cast(),
      );
      v.set_len(len_written.try_into().unwrap());
    }
    String::from_utf8_lossy(&v).into_owned()
  }

  pub fn get_attrib(&self, str: &CStr) -> VertexAttrib {
    let ans = GetAttribLocation(self.0, str.as_ptr());
    if ans == -1 {
      panic!("Failed to get {} attribute of program {:?}", str.to_str().unwrap(), self)
    }
    VertexAttrib(ans)
  } 

  pub fn use_program(&self) {
    unsafe { gl::UseProgram(self.0) };
    check_gl_errors();
  }

  pub fn delete(self) {
    unsafe { gl::DeleteProgram(self.0) };
    check_gl_errors();
  }

  pub fn get_uniform(&self, str: &CStr) -> Result<UniformLocation, String> {
    let uniform = GetUniformLocation(self.0, str.as_ptr());
    if uniform != -1 {
      Ok(UniformLocation(uniform))
    } else {
      Err(format!("Failed to get uniform {} from program {:?}", str.to_str().unwrap(), self))
    }
  }

  pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
    let p =
      Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
    let v = Shader::from_source(ShaderType::Vertex, vert)
      .map_err(|e| format!("Vertex Compile Error: {}", e))?;
    let f = Shader::from_source(ShaderType::Fragment, frag)
      .map_err(|e| format!("Fragment Compile Error: {}", e))?;
    p.attach_shader(&v);
    p.attach_shader(&f);
    p.link_program();
    v.delete();
    f.delete();
    if p.link_success() {
      Ok(p)
    } else {
      let out = format!("Program Link Error: {}", p.info_log());
      p.delete();
      Err(out)
    }
  }
}