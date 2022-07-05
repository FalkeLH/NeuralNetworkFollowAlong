fn main() {
    // Fetch basic inputs, weights and biases
    let x = vec![
        vec![1., 2., 3., 2.5],
        vec![2., 5., -1., 2.],
        vec![-1.5, 2.7, 3.3, -0.8],
    ];

    let weights = vec![
        vec![0.2, 0.8, -0.5, 1.],
        vec![0.5, -0.91, 0.26, -0.5],
        vec![-0.26, -0.27, 0.17, 0.87],
    ];
    let bias = vec![2., 3., 0.5];

    let weights2 = vec![
        vec![0.1, -0.14, 0.5],
        vec![-0.5, 0.12, -0.33],
        vec![-0.44, 0.73, -0.13],
    ];
    let bias2 = vec![-1., 2., -0.5];

    // Make sure there are an equal amount of biases and weights. Calculate the final output as well
    assert_eq!(weights.len(), bias.len());
    let layer1_output = calc_output(&x, &weights, &bias);
    let layer2_output = calc_output(&layer1_output, &weights2, &bias2);

    println!("{:?}", layer1_output);
    println!("{:?}", layer2_output);
}

fn calc_output<'a>(
    input: &'a Vec<Vec<f64>>,
    weight: &'a Vec<Vec<f64>>,
    bias: &'a Vec<f64>,
) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = vec![vec![0.; weight.len()]; input.len()];
    for i in 0..weight.len() {
        for j in 0..input.len() {
            for k in 0..weight[i].len() {
                output[j][i] += input[j][k] * weight[i][k];
            }
            output[j][i] += bias[i]
        }
    }
    output
}
