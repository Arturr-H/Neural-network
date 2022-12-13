# Sebastian Lague's Neural Network Video

## How did he structure his network layers?

He stores the `weight` values for incoming conenctions, in a two dimensional array. The 
"rows" in the matrix are each `neurons`, containing their `weight` to every neuron in the
next layer.

He stores the `bias` values in a 1dim array, for every neuron in the CURRENT layer.

Outgoing nodes are the nodes in the current layer, and incoming is the previous layer.
The `input` layer doesn't have any incoming nodes obviously, it only needs to represent
some values that the network need to calculate, it's the `input`.

### Constructor

He then has implemented a constructor for that Layer structure, that takes the number
of incoming nodes, and outgoing nodes as two parameters and inside of that function
sets the self.weights (two dim array) and self.biases.

### Calculate inputs method
After this, he creates yet another method inside the structure named `calculate_outputs`,
with a parameter taking a vector of f64 as input (`Vec<f64>`) and names them `input`.
He begins by initializing a mutable array called `weighted_inputs` with capacity 
`numNodesOut`.

He then initializes a for-loop with the range 0..`numNodesOut` and names the index value
`node_out`. (Basically iterates over all outgoing nodes) and inside of the loop-body, he
creates a mutable f64 called `weighted_input` which starts with the value self.biases[nodeOut]

We then nest another for loop inside the previous loop-body, with the range 0..`numNodesIn`
and calls the index value `node_in`. He then appends to the `weighted_input` float with
`inputs`[`node_in`] * `weights`[`node_in`, `node_out`] resulting in this expression:

weighted_input += inputs[node_in] * weights[node_in, node_out];

We basically grab the previous neurons' weight connected to the current neuron we're on. And
after that, he pushes to the first mutable Vector of f64:s we created in the beginning of this
function (`weighted_inputs`) to `weighted_input` which we previously set.

After all of that we return `weighted_inputs`

## Neural network structure

Has a Vector of `Layer` called `layers` which we previously mentioned. Implements a constructor
that takes a vector of unsigned integers as input called `layer_sizes` and initializes all layers
with these corresponding sizes.

The function body begins with creating a variable to store all layers (initialized with the
::with_capacity() function). Then he iterates over 0..size_of_that_array with an index. Inside
of the for-loop body he pushes a new layer constructed with the two parameters: layer_sizes[index], 
layer_sizes[index + 1] And I guess he then sets the inner value of the network structure (layers).