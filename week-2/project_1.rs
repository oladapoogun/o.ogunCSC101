fn main() {
    let p: f64 = 520_000_000.0; // Principal amount in Naira
    let r: f64 = 10.0;          // Rate of interest per annum in percentage
    let n: f64 = 5.0;           // Time period in years

    // Calculate total amount (A)
    let a = p * (1.0 + r / 100.0).powf(n);

    // Calculate compound interest (CI)
    let ci = a - p;

    // Output the results
    println!("Total Amount (A): {:.2} Naira", a);
    println!("Compound Interest (CI): {:.2} Naira", ci);
}
