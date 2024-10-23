fn main() {
    // Define the amounts as floating-point values
    let amounts = [
        450_000.00,
        1_500_000.00,
        750_000.00,
        2_850_000.00,
        250_000.00,
    ];

    // Calculate the sum
    let sum: f64 = amounts.iter().sum();
    
    // Calculate the average
    let average = sum / amounts.len() as f64;

    // Print the results
    println!("Total Amount: {:.2}", sum);
    println!("Average Amount: {:.2}", average);
}
