fn main() {
    // Quantities of items
    let q1 = 2; // TOSHIBA
    let q2 = 1; // MAC
    let q3 = 3; // HP
    let q4 = 3; // DELL
    let q5 = 1; // ACER

    // Amounts of items
    let a1 = 450_000.00; // TOSHIBA
    let a2 = 1_500_000.00; // MAC
    let a3 = 750_000.00; // HP
    let a4 = 2_850_000.00; // DELL
    let a5 = 250_000.00; // ACER

    // Calculate total amount, converting integers to f64 for multiplication
    let total_amount = 
        (q1 as f64 * a1) + 
        (q2 as f64 * a2) + 
        (q3 as f64 * a3) + 
        (q4 as f64 * a4) + 
        (q5 as f64 * a5);

    // Calculate total quantity
    let total_quantity = q1 + q2 + q3 + q4 + q5;

    // Calculate average
    let average = total_amount / total_quantity as f64;

    // Print results
    println!("Total Amount: {:.2}", total_amount);
    println!("Average Amount per Quantity: {:.2}", average);
}
