/*- Imports -*/
use crate::layer::Layer;
use serde;
use serde_json;
use serde_derive::{ Serialize, Deserialize };
use std::fs;

/*- Constants -*/


/*- Main neural network struct -*/
#[derive(Clone)]
pub struct NeuralNetwork {
    /*- All layers -*/
    layers: Vec<Layer>,
}

/*- Datapoint for storing labels and data for training -*/
#[derive(Debug)]
pub struct DataPoint {
    /// The inputs to the layer, aka the data
    pub data: Vec<f64>,

    /// The expected outputs of the layer, aka the labels
    pub labels: Vec<f64>,
}
impl DataPoint {
    pub fn from(data: &Vec<Vec<f64>>, labels: &Vec<Vec<f64>>) -> Vec<Self> {
        let mut datapoints:Vec<Self> = Vec::with_capacity(data.len());

        /*- Iterate -*/
        for i in 0..data.len() {
            datapoints.push(
                Self {
                    data: data[i].to_vec(),
                    labels: labels[i].to_vec()
                }
            );
        };

        datapoints
    }
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
        let outputs:Vec<f64> = self.calculate_outputs(&datapoint.data);
        let _output_layer = self.get_output_layer();

        /*- The cost for this datapoint -*/
        let mut cost:f64 = 0.0;

        /*- Iterate -*/
        for node_out in 0..outputs.len() {
            cost += Layer::node_cost(outputs[node_out], datapoint.labels[node_out]);
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
        println!("cost: {original_cost}");

        /*- Iterate over layers -*/
        let mut layer_index = 0;
        for layer in self.layers.clone() {
            /*- Calculate cost gradients for current weights -*/
            for node_in in 0..layer.nodes_in {
                for node_out in 0..layer.nodes_out {
                    /*
                        I'll explain the H constant now, it basically changes a specific weight by a
                        very very little amount, and runs the output with that little change on that
                        one weight. After we got a result, we compare that to the cost without that
                        delta change and see if it got any closer to the expected values.
                    */
                    self.layers[layer_index].weights[node_in][node_out] += H;

                    /*- Diffrence in current cost and original -*/
                    let delta_cost:f64 = self.cost(&training_data) - original_cost;

                    self.layers[layer_index].weights[node_in][node_out] -= H;
                    self.layers[layer_index].cost_gradient_weights[node_in][node_out] = delta_cost / H;
                };
            };

            /*- Calculate cost gradient for biases in the current layer -*/
            for index in 0..layer.biases.len() {
                self.layers[layer_index].biases[index] += H;

                /*- Diffrence in current cost and original -*/
                let delta_cost:f64 = self.cost(&training_data) - original_cost;

                self.layers[layer_index].biases[index] -= H;
                self.layers[layer_index].cost_gradient_biases[index] = delta_cost / H;
            };

            /*- Increment layer index -*/
            layer_index += 1;
        };

        self.apply_all_gradients(learn_rate);
    }

    /*- Simply loops through all layers and applies their gradients -*/
    pub fn apply_all_gradients(&mut self, learn_rate:f64) -> () {
        for layer in self.layers.iter_mut() {
            layer.apply_gradients(learn_rate);
        };
    }
    // pub fn update_all_gradiends(&mut self, datapoint: DataPoint) -> () {
    //     self.calculate_outputs(&datapoint.data);

    //     let output_layer:&Layer = self.get_output_layer();
    //     let node_values:Vec<f64> = output_layer.calculate_output_layer_node_values(datapoint.labels);
    // }
}


// Imagine that I have a struct in rust, called MyStruct that has a field named items, that struct also has a method that looks something like this: 
// impl MyStruct { pub fn do_something(&self) -> i32 { 5 } }
// How do i mutably iterate through the items (using .iter_mut()) in the items field and call the do_something method inside of the iterator?
// When I try to do this the compiler says that I can't borrow self as mutable (iter_mut) whilst also using it immutably (do_something)