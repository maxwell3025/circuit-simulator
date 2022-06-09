extern crate sdl2;

mod gui;
mod logic;
use gui::button::Button;
use gui::user_interface::Component;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::time::Duration;

use gui::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut root_component = Button::new(
        Rect::new(0,0,100,100), 
        Color::RED,
        Color::WHITE);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        (root_component.borrow_mut().deref_mut().deref_mut() as &mut dyn Component).update(&mut canvas);
        
        for event in event_pump.poll_iter() {
            (root_component.borrow_mut().deref_mut().deref_mut() as &mut dyn Component).handle_events(&event);
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}