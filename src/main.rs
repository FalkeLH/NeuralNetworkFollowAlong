fn main() {
    let inputs = vec![1.2, 5.1, 2.1];
    let weights = vec![3.1, 2.1, 8.7];
    let bias = 3.;
    
    let mut output = 0.;
    inputs
        .into_iter()
        .zip(weights)
        .map(|(i, w)| i * w)
        .collect::<Vec<f64>>()
        .into_iter()
        .for_each(|e| output += e);
    let output = output + bias;

    println!("{}", output);
}
