fn main() {
    // Given values
    let p: f64 = 210_000.0;  // Original price in Naira
    let r: f64 = 10.0;       // Depreciation rate in percentage per year
    let n: f64 = 3.0;        // Number of years

    // Calculate the depreciated value (A)
    let a = p * (1.0 - r / 100.0).powf(n);

    // Output the result
    println!("The value of the TV after 3 years: {:.2} Naira", a);
}
