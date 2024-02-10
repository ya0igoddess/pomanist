extern crate gl;

pub fn check_gl_errors() {
    #[cfg(debug_assertions)]
    unsafe {
        let error = gl::GetError();
        if error != 0 {
            panic!("Openg Errored with code {}", error);
        }
    }
}