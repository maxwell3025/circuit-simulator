use std::rc::*;
use std::cell::*;
use std::ops::Deref;

struct NodeOut{
    buffer: bool,
    value: Rc<RefCell<bool>>,
    x: i32,
    y: i32
}

struct NodeIn{
    value: Weak<RefCell<bool>>,
    x: i32,
    y: i32
}

impl NodeIn{
    pub fn read(&self) -> bool{
        match self.value.upgrade(){
            None =>{
                false
            }
            Some(x) => {
                *x.borrow()
            }
        }
    }
}

pub enum Gate{
    Buf{
        input: NodeIn,
        out: NodeOut
    },
    Not{
        input: NodeIn,
        out: NodeOut
    },
    And{
        input_1: NodeIn,
        input_2: NodeIn,
        out: NodeOut
    },
    Or{
        input_1: NodeIn,
        input_2: NodeIn,
        out: NodeOut
    },
    Xor{
        input_1: NodeIn,
        input_2: NodeIn,
        out: NodeOut
    }
}

impl Gate{
    pub fn update(&mut self){
        match self{
            Self::Buf{input, out}=>{
                out.buffer = input.read();
            },
            Self::Not{input, out}=>{
                out.buffer = !input.read();   
            },
            Self::And{input_1, input_2, out} => {
                out.buffer = input_1.read() & input_2.read();
            }
            Self::Or{input_1, input_2, out} => {
                out.buffer = input_1.read() | input_2.read();
            }
            Self::Xor{input_1, input_2, out} => {
                out.buffer = input_1.read() ^ input_2.read();
            }
        }
    }
}