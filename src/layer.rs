/*- Imports -*/
use crate::neuron::Neuron;

/*- Layer struct, used for storing neurons and it's main purpose is to give more functionality -*/
#[derive(Debug, Clone)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

/*- Method implementations -*/
impl Layer {
    /*- Constructor -*/
    pub fn new(repl:Neuron, count:usize) -> Self {
        Self {
            neurons: vec![repl;count as usize]
        }
    }

    /*- Returns the size of the layer -*/
    pub fn size(&self) -> usize {
        self.neurons.len()
    }

    /*- Create input layer -*/
    pub fn input(input:&[u8], next_layer_size:usize) -> Self {
        let mut neurons:Vec<Neuron> = Vec::with_capacity(input.len());
        for input in input {
            neurons.push(Neuron::specific(*input as f64, next_layer_size));
        }
        Self { neurons }
    }
}