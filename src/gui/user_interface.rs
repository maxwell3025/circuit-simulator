extern crate sdl2;

use std::rc::*;

use sdl2::rect::*;
use sdl2::event::*;
use sdl2::video::*;
use sdl2::render::*;

pub struct Component{
    bounds: Rect,
    children: Vec<Component>,
    offset: (u32, u32),
    on_event: Box<dyn FnMut(Event)>,
    draw_self: Box<dyn FnMut(&mut Canvas<Window>)>
}

impl Component{
    fn new(width: u32, height: u32)-> Self{
        Component{
            bounds: Rect::new(0, 0, width, height),
            children: Vec::new(),
            offset: (0, 0),
            on_event: Box::new(|x|println!("hello world!")),
            draw_self: Box::new(|x|println!("hello world!")),
        }
    }
    fn render(&mut self, canvas: &mut Canvas<Window>){
        (*self.draw_self)(canvas);
        for child in &mut self.children {
            child.render(canvas);
        }
    }
    fn add_child(self: &mut Self, child: Component){
        self.children.push(child);
    }
}