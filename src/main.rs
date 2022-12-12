/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports
)]

/*- Imports -*/
mod network;
mod neuron;
mod layer;

/*- Constants -*/


/*- Structs, enums & unions -*/


/*- Initialize -*/
fn main() -> () {
    let network = network::NeuralNetwork::new(1, &[4, 4], 2);

    dbg!(network);
}
