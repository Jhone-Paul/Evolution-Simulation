//dependencies
use rand::Rng;

// using this error crate 
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("got {got} inputs, but {expected} inputs were expected")]
    MismatchedInputSize {
        got: usize,
        expected: usize,
    },
}

// Define the Network, Layer, and Neuron structs
#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {

    pub fn random(layers: &<LayerTopology>) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    //getting rid of the rebinding is very rustic!
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {

        for layer in &self.layers {
            inputs = layer.propogate(inputs);
        }

        inputs
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {

    fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();

        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        outputs
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {

    fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &<f32>) -> f32 {

        if inputs.len() != self.weights.len() {
            return Err(Error::MismatchedInputSize {
                got: inputs.len(),
                expected: self.weights.len(),
            });
        }

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}

// generated add function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// generated test module 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
