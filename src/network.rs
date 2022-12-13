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
        /*- Set input to be size of first hidden (next layer) -*/
        let _input = Layer::new(Neuron::new(hidden[0]), input_len);

        /*- Set hidden -*/
        let mut _hidden:Vec<Layer> = Vec::with_capacity(hidden.len()-1);
        for (index, size) in hidden.iter().enumerate() {
            /*- Get next layer size -*/
            let next_size = match hidden.get(index+1) {
                Some(size) => size,
                None => &output
            };
            _hidden.push(Layer::new_hidden(next_size, size));
        };

        /*- Set output (0 weights because no layers exist after output) -*/
        let _output = Layer::new(Neuron::new(0), output);

        /*- Return -*/
        Self { _input, _hidden, _output }
    }

    /*- Builder pattern -*/
    pub fn set_input(mut self, input:&[u8]) -> Self {
        assert!(self._input.size() == input.len(), "Input size does not match the set network input size");

        /*- Set input -*/
        self._input = Layer::input(input, self._hidden[0].size());
        self
    }

    /*- Calculate -*/
    pub fn calculate_all_neurons(&mut self) -> () {
        let hidden = self._hidden.clone();

        /*- Calculate hidden -*/
        for index in 0..hidden.len() {
            let prev_layer;
            
            /*- Check if index is 0, because then we cant sub with 1 wich will mean input will be selected -*/
            if index == 0 { prev_layer = &self._input }
            else { prev_layer = &hidden.get(index-1).unwrap() };

            /*- Calculate neurons -*/
            self._hidden[index].calculate_neurons(&prev_layer);
        }

        /*- Calculate output -*/
        self._output.calculate_neurons(&self._hidden[self._hidden.len()-1]);
    }

    /*- Get output -*/
    pub fn get_output(&self) -> Vec<u8> {
        let mut output:Vec<u8> = Vec::with_capacity(self._output.size());

        /*- Get all output neurons -*/
        for neuron in &self._output.neurons {
            output.push(neuron.inner.round() as u8);
        };
        
        output
    }
}
