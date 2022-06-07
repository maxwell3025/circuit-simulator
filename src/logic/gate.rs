use std::collections::HashMap;
use std::rc::*;
use std::cell::*;

use nalgebra::Matrix2;
use nalgebra::geometry::*;
use nalgebra::linalg::*;

pub struct NodeIn{
    value: Weak<RefCell<NodeOut>>
}

impl NodeIn{
    pub fn read(&self) -> bool{
        match self.value.upgrade(){
            None =>{
                false
            }
            Some(x) => {
                x.borrow().value
            }
        }
    }
}

pub struct NodeOut{
    buffer: bool,
    value: bool
}

struct ComponentNodeIn{
    node: Rc<RefCell<NodeIn>>,
    pos: Point2<i32>
}

impl ComponentNodeIn{
    pub fn new(node: Rc<RefCell<NodeIn>>, pos: Point2<i32>) -> Self{
        ComponentNodeIn {node: node, pos: pos}
    }
}

struct ComponentNodeOut{
    node: Rc<RefCell<NodeOut>>,
    pos: Point2<i32>
}

impl ComponentNodeOut{
    pub fn new(node: Rc<RefCell<NodeOut>>, pos: Point2<i32>) -> Self{
        ComponentNodeOut {node: node, pos: pos}
    }
}

pub enum ComponentInternal{
    Buf{
        input: Rc<RefCell<NodeIn>>,
        output: Rc<RefCell<NodeOut>>
    },
    Not{
        input: Rc<RefCell<NodeIn>>,
        output: Rc<RefCell<NodeOut>>
    },
    And{
        input_1: Rc<RefCell<NodeIn>>,
        input_2: Rc<RefCell<NodeIn>>,
        output: Rc<RefCell<NodeOut>>
    },
    Or{
        input_1: Rc<RefCell<NodeIn>>,
        input_2: Rc<RefCell<NodeIn>>,
        output: Rc<RefCell<NodeOut>>
    },
    Xor{
        input_1: Rc<RefCell<NodeIn>>,
        input_2: Rc<RefCell<NodeIn>>,
        output: Rc<RefCell<NodeOut>>
    },
    IC {
        circuit: Circuit,
        input_positions: Vec<Point2<i32>>,
        output_positions: Vec<Point2<i32>>
    },
}

impl ComponentInternal{
    pub fn update(&mut self){
        match self{
            Self::Buf{input, output}=>{
                output.borrow_mut().buffer = input.borrow().read();
            },
            Self::Not{input, output}=>{
                output.borrow_mut().buffer = !input.borrow().read();   
            },
            Self::And{input_1, input_2, output} => {
                output.borrow_mut().buffer = input_1.borrow().read() & input_2.borrow().read();
            },
            Self::Or{input_1, input_2, output} => {
                output.borrow_mut().buffer = input_1.borrow().read() | input_2.borrow().read();
            },
            Self::Xor{input_1, input_2, output} => {
                output.borrow_mut().buffer = input_1.borrow().read() ^ input_2.borrow().read();
            },
            Self::IC {circuit, ..} => {
                circuit.update();
            }
        }
    }
    
    pub fn inputs(&mut self) -> Vec<Rc<RefCell<NodeIn>>>{
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
                circuit.input_nodes
                .iter()
                .enumerate()
                .filter(|&(index, _)|circuit.pub_input_indices.contains_key(&index))
                .map(|(_, node)| node.clone())
                .collect()
            }
        }
    }

    pub fn outputs(&mut self) -> Vec<Rc<RefCell<NodeOut>>>{
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
                circuit.output_nodes
                .iter()
                .enumerate()
                .filter(|&(index, _)|circuit.pub_output_indices.contains_key(&index))
                .map(|(_, node)| node.clone())
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
    input_nodes: Vec<Rc<RefCell<NodeIn>>>,
    output_nodes: Vec<Rc<RefCell<NodeOut>>>,
    pub_input_indices: HashMap<usize, Point2<i32>>,
    pub_output_indices: HashMap<usize, Point2<i32>>,
    components: Vec<Component>,
}

impl Circuit{
    pub fn add_component(&mut self, mut component: Component){
        self.input_nodes.append(&mut component.internal.inputs());
        self.output_nodes.append(&mut component.internal.outputs());
        self.components.push(component);
    }

    pub fn update(&mut self){
        for component in &mut self.components{
            component.internal.update();
        }

        for node in &mut self.output_nodes{
            let value = node.borrow().buffer;
            node.borrow_mut().value = value;
        }
    }
}