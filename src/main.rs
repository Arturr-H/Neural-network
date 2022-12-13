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
    let _network = NeuralNetwork::new(&[1, 4, 6, 2]);
}
