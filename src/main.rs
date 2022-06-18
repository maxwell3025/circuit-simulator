extern crate sdl2;

mod gui;
mod logic;
use gui::circuit_gui::CircuitGui;
use sdl2::mouse::MouseState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use ::std::thread::sleep;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_loader = canvas.texture_creator();
    let mut circuit_menu = CircuitGui::init(&texture_loader);
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let mouse_state = MouseState::new(&event_pump);

        for event in event_pump.poll_iter() {
            circuit_menu.handle_event(event.clone(), mouse_state);
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        circuit_menu.render(&mut canvas);

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
