/*- Global allowings -*/



/*- Imports -*/
use crate::{ layer::Layer, neuron::Neuron };


/*- Constants -*/


/*- Main network struct containing weights, biases and more -*/
#[derive(Debug)]
pub struct NeuralNetwork {
    /*- Inputs -*/
    pub _input: Layer,

    /*- Hidden -*/
    pub _hidden: Vec<Layer>,

    /*- Output -*/
    // TODO: Maybe this should not be a layer, rather a vec of f64:s?
    pub _output: Layer
}

/*- Method implementations -*/
impl NeuralNetwork {
    /*- Constructor -*/
    pub fn new(input_len:usize, hidden:&[usize], output:usize) -> Self {
        /*- Set input -*/
        let _input = Layer::new(Neuron::new(), input_len);

        /*- Set hidden -*/
        let mut _hidden:Vec<Layer> = Vec::with_capacity(hidden.len()-1);
        for size in hidden {
            _hidden.push(Layer::new(Neuron::new(), *size));
        };

        /*- Set output -*/
        let _output = Layer::new(Neuron::new(), output);

        /*- Return -*/
        Self { _input, _hidden, _output }
    }
}