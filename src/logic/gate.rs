use std::collections::*;
use std::rc::*;
use std::cell::*;

use nalgebra::geometry::*;

type SharedBool = Rc<Cell<bool>>;
pub enum ComponentInternal{
    Buf{
        input: Option<SharedBool>,
        output_buffer: bool,
        output: SharedBool
    },
    Not{
        input: Option<SharedBool>,
        output_buffer: bool,
        output: SharedBool
    },
    And{
        input_1: Option<SharedBool>,
        input_2: Option<SharedBool>,
        output_buffer: bool,
        output: SharedBool
    },
    Or{
        input_1: Option<SharedBool>,
        input_2: Option<SharedBool>,
        output_buffer: bool,
        output: SharedBool
    },
    Xor{
        input_1: Option<SharedBool>,
        input_2: Option<SharedBool>,
        output_buffer: bool,
        output: SharedBool
    },
    IC {
        circuit: Box<Circuit>,
        input_positions: Vec<Point2<i32>>,
        output_positions: Vec<Point2<i32>>
    },
}

impl ComponentInternal{
    pub fn update(&mut self){
        match self{
            Self::Buf{input, output_buffer, ..}=>{
                *output_buffer = input.as_ref().map_or(false, |a| a.get());
            },
            Self::Not{input, output_buffer, ..}=>{
                *output_buffer = !input.as_ref().map_or(false, |a| a.get());   
            },
            Self::And{input_1, input_2, output_buffer, ..} => {
                *output_buffer = input_1.as_ref().map_or(false, |a| a.get()) & input_2.as_ref().map_or(false, |a| a.get());
            },
            Self::Or{input_1, input_2, output_buffer, ..} => {
                *output_buffer = input_1.as_ref().map_or(false, |a| a.get()) | input_2.as_ref().map_or(false, |a| a.get());
            },
            Self::Xor{input_1, input_2, output_buffer, ..} => {
                *output_buffer = input_1.as_ref().map_or(false, |a| a.get()) ^ input_2.as_ref().map_or(false, |a| a.get());
            },
            Self::IC {circuit, ..} => {
                circuit.tick();
            }
        }
    }
    pub fn swap(&mut self){

    }
    
    pub fn inputs(&mut self) -> Vec<Option<SharedBool>>{
        match self {
            Self::Buf{input, ..}=>{
                vec![input.clone()]
            },
            Self::Not{input, ..}=>{
                vec![input.clone()]
            },
            Self::And{input_1, input_2, ..} => {
                vec![input_1.clone(), input_2.clone()]
            },
            Self::Or{input_1, input_2, ..} => {
                vec![input_1.clone(), input_2.clone()]
            },
            Self::Xor{input_1, input_2, ..} => {
                vec![input_1.clone(), input_2.clone()]
            },
            Self::IC {circuit, ..} => {
                circuit.inputs.iter()
                .map(|a| a.clone())
                .collect()
            }
        }
    }

    pub fn outputs(&mut self) -> Vec<SharedBool>{
        match self {
            Self::Buf{output, ..}=>{
                vec![output.clone()]
            },
            Self::Not{output, ..}=>{
                vec![output.clone()]
            },
            Self::And{output, ..} => {
                vec![output.clone()]
            },
            Self::Or{output, ..} => {
                vec![output.clone()]
            },
            Self::Xor{output, ..} => {
                vec![output.clone()]
            },
            Self::IC {circuit, ..} => {
                circuit.outputs.iter()
                .map(|a| a.clone())
                .collect()
            }
        }
    }
}

struct Component{
    internal: ComponentInternal,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    orientation: Isometry2<i32>
}

struct Circuit{
    //these should contain all nodes in the comonents
    nodes: Vec<SharedBool>,
    inputs: Vec<Option<SharedBool>>,
    //should be idenstical to inputs, but maps Nones to constant false
    inputs_internal: Vec<SharedBool>,
    outputs: Vec<SharedBool>,
    components: Vec<Component>,
    width: u32,
    height: u32
}

impl Circuit{
    pub fn new(width: u32, height: u32) -> Self{
        Circuit {
            nodes: Vec::new(),
            inputs: Vec::new(),
            inputs_internal: Vec::new(),
            outputs: Vec::new(),
            components: Vec::new(),
            width: width,
            height: height
        }
    }

    pub fn add_component(&mut self, mut component: Component){
        self.nodes.append(&mut component.internal.outputs());
        self.components.push(component);
    }

    pub fn tick(&mut self){
        for component in &mut self.components{
            component.internal.update();
        }
    }
}