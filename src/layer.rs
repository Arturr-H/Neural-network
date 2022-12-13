/*- Imports -*/

/*- Layer struct, used for storing neurons and it's main purpose is to give more functionality -*/
#[derive(Debug, Clone)]
pub struct Layer {
    /*- Weights -*/
    weights: Vec<Vec<f64>>,

    /*- Biases -*/
    biases: Vec<f64>,

    /*- Node count -*/
    nodes_in: usize,
    nodes_out: usize,
}

/*- Method implementations -*/
impl Layer {
    /*- Constructor -*/
    pub fn new(nodes_in:usize, nodes_out:usize) -> Self {
        Self { 
            /*- Initialize with 0.0 -*/
            weights: vec![vec![0.0; nodes_out]; nodes_in],
            biases:  vec![0.0; nodes_out],

            /*- Node count -*/
            nodes_in, nodes_out
        }
    }

    /*- Calculate output of layer -*/
    pub fn calculate_output(&self, inputs: Vec<f64>) -> Vec<f64> {
        let mut weighted_inputs:Vec<f64> = Vec::with_capacity(self.nodes_out);

        /*- Iterate -*/
        for node_out in 0..self.nodes_out {
            
            /*- Create value -*/
            let mut weighted_input:f64 = self.biases[node_out];

            /*- Nest again -*/
            for node_in in 0..self.nodes_in {
                weighted_input += inputs[node_in] * self.weights[node_in][node_out];
            };

            /*- Push to end -*/
            weighted_inputs.push(weighted_input);
        };

        /*- Return -*/
        weighted_inputs
    }
}