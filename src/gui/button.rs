use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::user_interface::Component;
use std::collections::HashMap;
use std::rc::*;
use std::cell::*;

pub struct Button{
    parent: Option<Rc<RefCell<Box<dyn Component>>>>,
    bounds: Rect,
    pressed: bool,
    color_normal: Color,
    color_pressed: Color
}

impl Button{
    /// Creates new button with no parents.
    /// 
    /// This should typically be used before adding it as a child of another component, but it could also be used to create a root.
    pub fn new(bounds: Rect, color_normal: Color, color_pressed: Color) -> Rc<RefCell<Box<Button>>>{
        Rc::new(RefCell::new(Box::new(Button{
            parent: None,
            bounds,
            pressed: false,
            color_normal,
            color_pressed
        })))
    }

    pub fn pressed(&self) -> bool{
        self.pressed
    }
}

impl Component for Button{
    fn children(&mut self) -> HashMap<String, Rc<RefCell<Box<dyn Component>>>> {
        HashMap::new()
    }

    fn parent(&mut self) -> Option<Rc<RefCell<Box<dyn Component>>>> {
        self.parent.clone()
    }

    fn bounds(&mut self) -> &mut Rect {
        &mut self.bounds
    }

    fn render(&mut self, renderer: &mut Canvas<Window>) {
        renderer.set_draw_color(if self.pressed {self.color_pressed} else {self.color_normal});
        renderer.fill_rect(self.bounds).expect("Failed to draw Button");
    }

    fn handle_events(&mut self, event: &Event) {
        match *event{
            Event::MouseButtonDown {x, y, ..} => {
                if self.bounds.contains_point(Point::new(x, y)){
                    self.pressed = true;
                }
            }
            Event::MouseButtonUp{..} => {self.pressed = false;}
            _ => {}
        }
    }
}