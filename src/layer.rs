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
    pub fn new_hidden(next_size:&usize, count:&usize) -> Self {

        /*- Create neurons -*/
        let mut neurons:Vec<Neuron> = Vec::with_capacity(*count as usize);

        /*- Create neurons -*/
        for _ in 0..*count {
            neurons.push(Neuron::new(*next_size)); // next size is size of next layer which will be used for the amount of weights
        };

        /*- Return -*/
        Self { neurons }
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

    /*- Calculations -*/
    pub fn calculate_neurons(&mut self, prev_layer:&Layer) -> () {

        /*- Change neuron inner values. -*/
        for (index, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.calculate_inner(&prev_layer, index);
        }
    }
}