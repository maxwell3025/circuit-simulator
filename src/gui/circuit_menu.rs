use sdl2::{render::{Texture, Canvas, TextureCreator, WindowCanvas}, video::{Window, WindowContext}, rect::Rect, event::Event, EventPump, mouse::MouseState, image::LoadTexture};

const ICON_SIZE: i32 = 200;
const SCROLL_SENSITIVITY: i32 = ICON_SIZE/4;

pub struct CircuitMenu<'a>{
    pub icons: Vec<Texture<'a>>,
    scroll_position: i32,
}

impl<'a> CircuitMenu<'a>{
    pub fn init(texture_loader: &'a TextureCreator<WindowContext>) -> Self{
        let out = CircuitMenu {
            icons: vec![texture_loader.load_texture("smile.png").unwrap()],
            scroll_position: 0,
        };
        out
    }

    pub fn render(&self, canvas: &mut Canvas<Window>){
        for (index, texture) in self.icons.iter().enumerate(){
            canvas.copy(
                texture, 
                None, 
                Some(Rect::new(
                    0,
                    (index as i32) * ICON_SIZE + self.scroll_position,
                    ICON_SIZE as u32,
                    ICON_SIZE as u32
                ))
            ).unwrap();
        }
    }
    pub fn handle_event(&mut self, event: Event, mouse_state: MouseState){
        match event{
            Event::MouseWheel {y, ..}=>{
                if mouse_state.x() < ICON_SIZE{
                    self.scroll_position += y * SCROLL_SENSITIVITY;
                }
            }
            _=>{}
        }
    }
}