fn main() {
    // Fetch basic inputs, weights and biases
    let inputs = vec![1., 2., 3., 2.5];
    let weights = vec![
        vec![0.2, 0.8, -0.5, 1.],
        vec![0.5, -0.91, 0.26, -0.5],
        vec![-0.26, -0.27, 0.17, 0.87],
    ];
    let bias = vec![2., 3., 0.5];

    // Make sure there are an equal amount of biases and weights. Calculate the final output as well
    assert_eq!(weights.len(), bias.len());
    let mut output: Vec<f64> = vec![0.; weights.len()];
    for i in 0..weights.len() {
        for j in 0..weights[i].len() {
            output[i] += inputs[j] * weights[i][j];
        }
        output[i] += bias[i]
    }

    println!("{:?}", output);
}
