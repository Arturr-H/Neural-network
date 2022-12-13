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
    let mut network = NeuralNetwork::new(4, &[6, 5], 2)
        .set_input(&[ 1, 0, 0, 1 ]);

    network.calculate_all_neurons();
        
    dbg!(&network);
    println!("{:?}", network.get_output());

}
