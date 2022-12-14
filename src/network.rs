/*- Imports -*/

/*- Constants -*/

use crate::layer::Layer;

/*- Main neural network struct -*/
pub struct NeuralNetwork {
    /*- All layers -*/
    layers: Vec<Layer>,
}

/*- Datapoint for storing labels and data for training -*/
pub struct DataPoint {
    /*- Inputs -*/
    inputs: Vec<f64>,

    /*- What we want the network to output -*/
    expected_outputs: Vec<f64>,
}

/*- Method implementations -*/
impl NeuralNetwork {
    /*- Constructor -*/
    pub fn new(layer_sizes:&[usize]) -> Self {
        let len = layer_sizes.len();
        let mut layers:Vec<Layer> = Vec::with_capacity(len);

        /*- Iterate and construct layers -*/
        for i in 0..len-1 {
            layers.push(
                Layer::new(layer_sizes[i], layer_sizes[i + 1])
            );
        };

        Self { layers }
    }

    /*- Getters -*/
    pub fn get_output_layer(&self) -> &Layer {
        &self.layers[self.layers.len() - 1]
    }

    /*- Calculate output of network - I've written docs in the readme.md file -*/
    pub fn calculate_outputs(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut inps_:Vec<f64> = inputs.to_vec();

        /*- Forward pass -*/
        for layer in &self.layers {
            inps_ = layer.calculate_output(inps_.to_vec());
        };

        /*- Return -*/
        inps_.to_owned()
    }

    /*- Cost function -*/
    pub fn single_cost(&self, datapoint:&DataPoint) -> f64 {
        let outputs:Vec<f64> = self.calculate_outputs(&datapoint.inputs);
        let output_layer = self.get_output_layer();

        /*- The cost for this datapoint -*/
        let mut cost:f64 = 0.0;

        /*- Iterate -*/
        for node_out in 0..outputs.len() {
            cost += output_layer.node_cost(outputs[node_out], datapoint.expected_outputs[node_out]);
        };

        cost
    }
    pub fn cost(&self, datapoints:&[DataPoint]) -> f64 {
        /*- Cost -*/
        let mut total_cost:f64 = f64::default();

        /*- Add all costs -*/
        for dp in datapoints {
            total_cost += self.single_cost(dp);
        };

        /*- Return avg -*/
        total_cost / datapoints.len() as f64
    }
}