use sdl2::event::Event;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use super::circuit_menu::CircuitMenu;
use super::design_pane::DesignPane;

pub struct CircuitGui<'a>{
    menu: CircuitMenu<'a>,
    design_pane: DesignPane,
}

impl<'a> CircuitGui<'a>{
    pub fn init(texture_loader: &'a TextureCreator<WindowContext>) -> Self{
        CircuitGui{
            menu: CircuitMenu::init(texture_loader),
            design_pane: DesignPane::init(),
        }
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        self.menu.render(canvas);
        self.design_pane.render(canvas);
        canvas.present();
    }
    pub fn handle_event(&mut self, event: Event, mouse_state: MouseState){
        self.menu.handle_event(event.clone(), mouse_state);
        self.design_pane.handle_event(event.clone(), mouse_state);
    }
}