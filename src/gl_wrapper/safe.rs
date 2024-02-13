#[allow(non_snake_case)]
pub mod safe_gl {
    use  gl::*;

    use crate::gl_wrapper::check_gl_errors::check_gl_errors;

    pub fn print_context_debug_info() {
        unsafe {
            let mut array_buffer = 0;
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut array_buffer);
            let mut element_array_buffer = 0;
            gl::GetIntegerv(gl::ELEMENT_ARRAY_BUFFER_BINDING, &mut element_array_buffer);
            let mut vao = 0;
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut vao);
            let mut texture = 0;
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut texture);

            println!("
            CURRENT GL CONTEXT:
            VAO {:?}, 
            ARRAY_BUFFER {}, 
            ELEMENT_ARRAY_BUFFER {}, 
            TEXTURE {}", 
            vao, array_buffer, element_array_buffer, texture);
        }
    }

    pub fn ClearColor(red: types::GLfloat, green: gl::types::GLfloat, blue: gl::types::GLfloat, alpha: gl::types::GLfloat) -> () {
        unsafe { gl::ClearColor(red, green, blue, alpha); }
        check_gl_errors();
    }

    pub fn Clear(mask: types::GLbitfield) -> () {
        unsafe { gl::Clear(mask) }
        check_gl_errors();
    }

    pub fn EnableVertexAttribArray(index: types::GLuint) -> () {
        unsafe { gl::EnableVertexAttribArray(index) }
        check_gl_errors();
    }

    pub fn TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () {
        unsafe { gl::TexParameteri(target, pname, param) }
        check_gl_errors();
    }

    pub fn TexImage2D(
        target: types::GLenum, 
        level: types::GLint, 
        internalformat: types::GLint, 
        width: types::GLsizei, 
        height: types::GLsizei, 
        border: types::GLint, 
        format: types::GLenum, 
        type_: types::GLenum, 
        pixels: &[u8]
    ) -> () {
        unsafe { gl::TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels.as_ptr().cast() ) }
        check_gl_errors();
    }

    pub fn GetAttribLocation(program: u32, name: *const i8) -> gl::types::GLint {
        let ans = unsafe { gl::GetAttribLocation(program, name) };
        check_gl_errors();
        ans
    }

    pub fn GetUniformLocation(program: u32, name: *const i8) -> gl::types::GLint {
        let ans = unsafe { gl::GetUniformLocation(program, name) };
        check_gl_errors();
        ans
    }

    pub fn DrawElements(mode: gl::types::GLenum, count: i32, type_: gl::types::GLuint, indicies: u32) {
        unsafe { gl::DrawElements(mode, count, type_, indicies as *const _) }
        check_gl_errors()                                                                                                                                                                                                                                                                                                                                                    
    }

    pub fn ProgramUniform3(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) {
        unsafe { gl::ProgramUniform3f(program, location, v0, v1, v2) }
        check_gl_errors()
    }
}                     