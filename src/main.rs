/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports
)]

/*- Imports -*/
mod network;
mod layer;
use network::{NeuralNetwork, load_json_data};

/*- Constants -*/

/*- Structs, enums & unions -*/

/*- Initialize -*/
fn main() -> () {
    let mut network = NeuralNetwork::new(&[1, 4, 6, 2]);
    let training_data = load_json_data();

    let learn_rate = 0.1;

    /*- Train -*/
    for i in 0..100 {
        println!("{i}");
        network.learn(&training_data, learn_rate);
    };

    /*- Test -*/
    let test_data = vec![1.0, 1.0, 0.0, 1.0, 1.0];
    println!("{:?}", network.calculate_outputs(&test_data));


    // dbg!(network);
}
