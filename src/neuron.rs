/*- Imports -*/
use rand;

use crate::layer::Layer;

/*- The neurons contaning data such as biases, weights and
    values. Nxt is the amount of neurons in the next layer -*/
#[derive(Clone)]
pub struct Neuron {
    /*- The values of the inputs and the output of the neuron are passed to the a
        forwarding method as arguments, and they are computed within the method.
        This means that the values are not stored within the Neuron struct itself,
        but they are only used temporarily during the computation. Here we use the
        inner value just for visualization -*/
    pub inner: f64,

    /*- Weights coupled to other neurons -*/
    pub weights: Vec<f64>,

    /*- Biases coupled to *this* neurons -*/
    pub bias: f64
}

/*- Method implementations -*/
impl Neuron {
    /*- Constructor -*/
    pub fn new(next_layer_size:usize) -> Self {
        Self {
            inner: 0.,
            weights: Self::initialize_weights(next_layer_size),
            bias: Self::initialize_bias()
        }
    }
    pub fn specific(inner:f64, next_layer_size:usize) -> Self {
        Self {
            inner,
            weights: Self::initialize_weights(next_layer_size),
            bias: Self::initialize_bias()
        }
    }

    /*- Initializers -*/
    fn initialize_bias() -> f64 {
        rand::random::<f64>() * 2. - 1.
    }
    fn initialize_weights(next_layer_size:usize) -> Vec<f64> {
        let mut weights:Vec<f64> = Vec::with_capacity(next_layer_size);
        for _ in 0..next_layer_size {
            weights.push(rand::random::<f64>() * 2. - 1.);
        }
        weights
    }

    /*- Calculations -*/
    pub fn calculate_inner(&mut self, prev_layer:&Layer, index_in_layer:usize) -> () {
        let mut inner:f64 = 0.;
        println!("{index_in_layer}");
        /*- Iterate over neurons in prev layer -*/
        for prev_neuron in prev_layer.neurons.iter() {
            /*- Multiply the inner value of the neuron with the weight of the neuron
                in the next layer -*/
            inner += prev_neuron.weights[index_in_layer] * prev_neuron.inner;
        }

        /*- Add the bias to the inner value & return -*/
        self.inner = inner + self.bias
    }
}

/*- Debug impl for cleaner visualization -*/
impl std::fmt::Debug for Neuron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "Neuron( {: ^8} | {: ^8} | {: ^8} )",
                format!("{:.3}i", self.inner),
                format!("{:?}w", self.weights.iter().map(|x| format!("{:.3}", x).parse::<f64>().unwrap()).collect::<Vec<f64>>()),
                format!("{:.3}b", self.bias)
            )
        )
    }
}
