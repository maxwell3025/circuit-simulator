extern crate sdl2;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::*;

use sdl2::rect::*;
use sdl2::event::*;
use sdl2::video::*;
use sdl2::render::*;

pub trait Component{
    fn parent(&mut self) -> Option<Rc<RefCell<Box<dyn Component>>>>;
    fn children(&mut self) -> HashMap<String, Rc<RefCell<Box<dyn Component>>>>;
    fn bounds(&mut self) -> &mut Rect;
    fn render(&mut self, renderer: &mut Canvas<Window>);
    fn handle_events(&mut self, event: &Event);
}

impl dyn Component{
    pub fn update(&mut self, renderer: &mut Canvas<Window>){
        self.render(renderer);
        for (_, child) in self.children(){
            child.borrow_mut().update(renderer);
        }
    }
    
    pub fn translate(&mut self, displacement: Point){
        self.bounds().offset(displacement.x, displacement.y);
        for (_, child) in self.children(){
            child.borrow_mut().translate(displacement);
        }
    }

    pub fn place(&mut self, position: Point){
        let displacement = position - self.bounds().top_left();
        self.translate(displacement);
    }
}

