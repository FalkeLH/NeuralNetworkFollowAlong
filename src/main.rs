fn main() {
    let inputs = vec![1., 2., 3., 4.];
    let weights = vec![0.2, 0.8, -0.5, 0.9];
    let bias = 2.;
    
    let mut output = 0.;
    inputs
        .into_iter()
        .zip(weights)
        .map(|(i, w)| i * w)
        .for_each(|e| output += e);
    let output = output + bias;

    println!("{}", output);
}
