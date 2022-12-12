/*- Imports -*/

/*- The neurons contaning data such as biases, weights and
    values. Nxt is the amount of neurons in the next layer -*/
#[derive(Clone)]
pub struct Neuron {
    /*- The values of the inputs and the output of the neuron are passed to the a
        forwarding method as arguments, and they are computed within the method.
        This means that the values are not stored within the Neuron struct itself,
        but they are only used temporarily during the computation. Here we use the
        inner value just for visualization -*/
    pub inner: f64,

    /*- Weights coupled to other neurons -*/
    pub weights: Vec<f64>,

    /*- Biases coupled to *this* neurons -*/
    pub biases: f64
}

/*- Method implementations -*/
impl Neuron {
    /*- Constructor -*/
    pub fn new() -> Self {
        Self { inner: 0., weights: Vec::new(), biases: 0. }
    }
}
impl std::fmt::Debug for Neuron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "N~ |{: ^8}|{: ^8}|{: ^8}|",
                self.inner, format!("{:?}", self.weights), self.biases
            )
        )
    }
}