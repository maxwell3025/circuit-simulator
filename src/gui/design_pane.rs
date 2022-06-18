use sdl2::{event::Event, render::Canvas, video::Window, mouse::MouseState};

pub struct DesignPane{

}

impl DesignPane{
    pub fn init() -> Self{
        DesignPane {

        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>){

    }
    pub fn handle_event(&mut self, event: Event, mouse_state: MouseState){
    
    }
}