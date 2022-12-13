/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports
)]

/*- Imports -*/
mod network;
mod neuron;
mod layer;
use network::NeuralNetwork;

/*- Constants -*/


/*- Structs, enums & unions -*/


/*- Initialize -*/
fn main() -> () {
    let network = NeuralNetwork::new(4, &[5, 4], 2)
        .set_input(&[ 1, 0, 0, 1 ]);

    dbg!(network);
}
