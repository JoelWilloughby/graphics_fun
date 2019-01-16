extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

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
    }

}
