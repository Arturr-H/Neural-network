/*- Imports -*/

use rand::Rng;

/*- Layer struct, used for storing neurons and it's main purpose is to give more functionality -*/
#[derive(Debug, Clone)]
pub struct Layer {
    /*- Weights -*/
    pub weights: Vec<Vec<f64>>,

    /*- Biases -*/
    pub biases: Vec<f64>,

    /*- Node count -*/
    pub nodes_in: usize,
    pub nodes_out: usize,

    /*- Costs -*/
    pub cost_gradient_weights: Vec<Vec<f64>>,
    pub cost_gradient_biases: Vec<f64>
}

/*- Method implementations -*/
impl Layer {
    /*- Constructor -*/
    pub fn new(nodes_in:usize, nodes_out:usize) -> Self {
        Self::weights_init(Self { 
            /*- Initialize with 0.0 -*/
            weights: vec![vec![0.0; nodes_out]; nodes_in],
            biases:  vec![0.0; nodes_out],

            /*- Node count -*/
            nodes_in, nodes_out,

            /*- Cost -*/
            cost_gradient_weights: vec![vec![0.0; nodes_out]; nodes_in],
            cost_gradient_biases:  vec![0.0; nodes_out],
        })
    }

    /*- Initialize all weights in the layer -*/
    pub fn weights_init(mut layer:Layer) -> Layer {
        let mut rng = rand::thread_rng();

        /*- Iterate over all weights -*/
        for node_out in 0..layer.nodes_out {
            for node_in in 0..layer.nodes_in {
                /*- Get random value between -1 and 1 -*/
                let random = rng.gen_range(-1.0..1.0);

                /*- Apply -*/
                layer.weights[node_in][node_out] = random / f64::sqrt(layer.nodes_in as f64);
            };
        };

        layer
    }

    /*- Calculate output of layer -*/
    pub fn calculate_output(&self, inputs: Vec<f64>) -> Vec<f64> {
        let mut activations:Vec<f64> = Vec::with_capacity(self.nodes_out);

        /*- Iterate -*/
        for node_out in 0..self.nodes_out {
            
            /*- Create value -*/
            let mut weighted_input:f64 = self.biases[node_out];

            /*- Nest again -*/
            for node_in in 0..self.nodes_in {
                weighted_input += inputs[node_in] * self.weights[node_in][node_out];
            };

            /*- Push to end -*/
            activations.push(Self::sigmoid(weighted_input));
        };

        /*- Return -*/
        activations
    }

    /*- Activation function -*/
    pub fn sigmoid(input: f64) -> f64 {
        1.0 / (1.0 + (-input).exp())
    }

    /*- Node cost function -*/
    pub fn node_cost(&self, output_activation:f64, expected_output:f64) -> f64 {
        (output_activation - expected_output).powi(2) / 2.0
    }

    /*- Apply gradients by using gradient descent. Update weights and biases -*/
    pub fn apply_gradients(&mut self, learn_rate: f64) -> () {
        /*- Iterate over current (outgoing) nodes -*/
        for node_out in 0..self.nodes_out {

            /*- Change bias -*/
            self.biases[node_out] -= self.cost_gradient_biases[node_out] * learn_rate;

            /*- Change all weights in all previous neurons -*/
            for node_in in 0..self.nodes_in {
                /*- Change bias -*/
                self.weights[node_in][node_out] -= self.cost_gradient_weights[node_in][node_out] * learn_rate;
            };
        };
    }
}