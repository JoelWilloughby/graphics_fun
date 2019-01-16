extern crate gl;
extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe{
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for _event in event_pump.poll_iter() {
            // User input
            match _event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        // Draw stuff
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

    }

}
