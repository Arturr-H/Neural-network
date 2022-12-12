/*- Imports -*/
use crate::neuron::Neuron;

/*- Layer struct, used for storing neurons and it's main purpose is to give more functionality -*/
#[derive(Debug)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

/*- Method implementations -*/
impl Layer {
    pub fn new(repl:Neuron, count:usize) -> Self {
        Self {
            neurons: vec![repl;count as usize]
        }
    }
}