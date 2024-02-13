extern crate sdl2;
extern crate gl;
extern crate ultraviolet;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use pomanist::gl_wrapper::safe::safe_gl as sgl;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    sgl::ClearColor(0.0, 0.0, 0.6, 1.0);
    sgl::Clear(gl::COLOR_BUFFER_BIT);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}