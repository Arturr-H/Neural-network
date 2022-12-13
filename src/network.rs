/*- Imports -*/

/*- Constants -*/

use crate::layer::Layer;

/*- Main neural network struct -*/
pub struct NeuralNetwork {
    /*- All layers -*/
    layer: Vec<Layer>,
}

/*- Method implementations -*/
impl NeuralNetwork {
    /*- Constructor -*/
    pub fn new(layer_sizes:&[usize]) -> () {
        let len = layer_sizes.len();
        let mut layers:Vec<Layer> = Vec::with_capacity(len);

        /*- Iterate and construct layers -*/
        for i in 0..len-1 {
            layers.push(
                Layer::new(layer_sizes[i], layer_sizes[i + 1])
            );
        };
    }
}