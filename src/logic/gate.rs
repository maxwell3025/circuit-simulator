use nalgebra::Complex;

#[derive(Clone, Copy)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl Direction{
    pub fn left(&self) -> Direction{
        match self{
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
    pub fn right(&self) -> Direction{
        match self{
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
}
#[derive(Clone, Copy)]
enum Tile{
    Blank,
    Wire{
        power: bool,
    },
    Cross{
        x_power: bool,
        y_power: bool,
    },
    Buffer{
        direction: Direction,
        output: bool,
    },
    Not{
        direction: Direction,
        output: bool,
    },
    And{
        direction: Direction,
        output: bool,
    },
    Or{
        direction: Direction,
        output: bool,
    },
    Xor{
        direction: Direction,
        output: bool,
    },
}

impl Default for Tile{
    fn default() -> Self{
        Tile::Blank
    }
}

impl Tile{
    pub fn direction(&self) -> Option<&Direction>{
        match self {
            Self::Buffer{direction, ..} |
            Self::Not{direction, ..} |
            Self::And{direction, ..} |
            Self::Or{direction, ..} |
            Self::Xor{direction, ..} => Some(direction),
            _ => None,
        }
    }
    
    pub fn direction_mut(&mut self) -> Option<&mut Direction>{
        match self {
            Self::Buffer{direction, ..} | 
            Self::Not{direction, ..} | 
            Self::And{direction, ..} | 
            Self::Or{direction, ..} | 
            Self::Xor{direction, ..} => Some(direction),
            _ => None,
        }
    }
    
    pub fn power(&self) -> Option<&bool>{
        match self {
            Self::Buffer{output, ..} |
            Self::Not{output, ..} |
            Self::And{output, ..} |
            Self::Or{output, ..} |
            Self::Xor{output, ..} => Some(output),
            _ => None,
        }
    }

    pub fn power_mut(&mut self) -> Option<&mut bool>{
        match self {
            Self::Buffer{output, ..} |
            Self::Not{output, ..} |
            Self::And{output, ..} |
            Self::Or{output, ..} |
            Self::Xor{output, ..} => Some(output),
            _ => None,
        }
    }
}

struct Circuit{
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Circuit{
    pub fn new(width: usize, height: usize) -> Self{
        Circuit { tiles: vec![Tile::Blank; width*height], width, height}
    }
    pub fn update(&mut self){
        //solve the gates
        for x in 0..self.width as i32{
            for y in 0..self.height as i32{
                match *self.get(x, y).unwrap() {
                    Tile::Buffer { direction, ..} => {
                        let signal = self.get_adj_power(x, y, direction);
                        if let Tile::Buffer {output, ..} = self.get(x, y).unwrap() {
                           *output = signal; 
                        }
                    }
                    Tile::Not { direction, ..} => {
                        let signal = self.get_adj_power(x, y, direction);
                        if let Tile::Buffer {output, ..} = self.get(x, y).unwrap() {
                           *output = !signal; 
                        }
                    }
                    Tile::And{ direction, ..} => {
                        let signal_left = self.get_adj_power(x, y, direction.left());
                        let signal_right = self.get_adj_power(x, y, direction.right());
                        if let Tile::Buffer {output, ..} = self.get(x, y).unwrap() {
                           *output = signal_left & signal_right; 
                        }
                    }
                    Tile::Or{ direction, ..} => {
                        let signal_left = self.get_adj_power(x, y, direction.left());
                        let signal_right = self.get_adj_power(x, y, direction.right());
                        if let Tile::Buffer {output, ..} = self.get(x, y).unwrap() {
                           *output = signal_left | signal_right; 
                        }
                    }
                    Tile::Xor{ direction, ..} => {
                        let signal_left = self.get_adj_power(x, y, direction.left());
                        let signal_right = self.get_adj_power(x, y, direction.right());
                        if let Tile::Buffer {output, ..} = self.get(x, y).unwrap() {
                           *output = signal_left ^ signal_right; 
                        }
                    }
                    _ => {}
                }
            }
        }
        //reset wires
        //set wires in front of gates
        //flood fill signal across wires
    }

    fn get_power(&self, x: i32, y: i32, face: Direction) -> bool{
        if (x<0) | (x>=self.width as i32) | (y<0) | (y>=self.height as i32){
            false
        }
        else{
            match self.tiles[(x as usize) + (y as usize) * self.width]{
                Tile::Blank => false,
                Tile::Buffer { output, .. } => output,
                Tile::Not { output, .. } => output,
                Tile::And { output, .. } => output,
                Tile::Or { output, .. } => output,
                Tile::Xor { output, .. } => output,
                Tile::Wire { power } => power,
                Tile::Cross { x_power, y_power} => {
                    match face {
                        Direction::Left | Direction::Right => x_power,
                        Direction::Up | Direction::Down => y_power,
                    }
                },
            }
        }
    }
        
    fn get_adj_power(&self, x: i32, y: i32, face: Direction) -> bool{
        match face{
            Direction::Up => self.get_power(x, y+1, Direction::Down), 
            Direction::Down => self.get_power(x, y-1, Direction::Up),
            Direction::Left => self.get_power(x-1, y, Direction::Right),
            Direction::Right => self.get_power(x+1, y, Direction::Left),
        }
    }
    fn get(&mut self, x: i32, y: i32) -> Option<&mut Tile>{
        if (x<0) | (x>=self.width as i32) | (y<0) | (y>=self.height as i32) {
            None
        } else {
            Some(&mut self.tiles[(x as usize)+(y as usize)*self.width])
        }
    }
}