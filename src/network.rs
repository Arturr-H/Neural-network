/*- Imports -*/
use crate::layer::Layer;
use serde;
use serde_json;
use serde_derive::{ Serialize, Deserialize };
use std::fs;

/*- Constants -*/
const PATH_TO_JSON_DATA: &'static str = "./learning_data.json";

/*- Main neural network struct -*/
pub struct NeuralNetwork {
    /*- All layers -*/
    layers: Vec<Layer>,
}

/*- Datapoint for storing labels and data for training -*/
#[derive(Debug)]
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
    pub fn cost(&self, datapoints:&Vec<DataPoint>) -> f64 {
        /*- Cost -*/
        let mut total_cost:f64 = f64::default();

        /*- Add all costs -*/
        for dp in datapoints {
            total_cost += self.single_cost(&dp);
        };

        /*- Return avg -*/
        total_cost / datapoints.len() as f64
    }

    /*- Training - run a single iteration of gradient descent -*/
    pub fn learn(&mut self, training_data:&Vec<DataPoint>, learn_rate:f64) -> () {
        const H:f64 = 0.0001;
        let original_cost:f64 = self.cost(&training_data);

        /*- Iterate over layers -*/
        let mut layers:Vec<Layer> = self.layers.clone();
        for layer in layers.iter_mut() {
            
            /*- Calculate cost gradients for current weights -*/
            for node_in in 0..layer.nodes_in {
                for node_out in 0..layer.nodes_out {
                    layer.weights[node_in][node_out] += H;

                    /*- Diffrence in current cost and original -*/
                    let delta_cost:f64 = self.cost(&training_data) - original_cost;

                    layer.weights[node_in][node_out] -= H;
                    layer.cost_gradient_weights[node_in][node_out] = delta_cost / H;
                };
            };

            /*- Calculate cost gradient for biases in the current layer -*/
            for index in 0..layer.biases.len() {
                layer.biases[index] += H;

                /*- Diffrence in current cost and original -*/
                let delta_cost:f64 = self.cost(&training_data) - original_cost;

                layer.biases[index] -= H;
                layer.cost_gradient_biases[index] = delta_cost / H;
            };
        };

        // TODO THIS MIGHT BE CAUSE OF BREAK
        self.layers = layers;
        self.apply_all_gradients(learn_rate);
    }

    /*- Simply loops through all layers and applies their gradients -*/
    pub fn apply_all_gradients(&mut self, learn_rate:f64) -> () {
        for layer in self.layers.iter_mut() {
            layer.apply_gradients(learn_rate);
        };
    }
}

/*- Load training data -*/
#[derive(Deserialize)]
struct JsonLoad { learning_data:Vec<Vec<Vec<f64>>> }
pub fn load_json_data() -> Vec<DataPoint> {
    let file = fs::read_to_string(PATH_TO_JSON_DATA).expect("Unable to read file");
    let json:JsonLoad = serde_json::from_str(&file).expect("Unable to parse json");
    let mut datapoints:Vec<DataPoint> = Vec::with_capacity(json.learning_data.len());

    /*- Iterate over all data points -*/
    for data_point in json.learning_data {
        let inputs = &data_point[0];
        let expected_outputs = &data_point[1];

        /*- Create datapoint -*/
        let dp = DataPoint { inputs: inputs.clone(), expected_outputs: expected_outputs.clone() };
        datapoints.push(dp);
    };

    datapoints
}