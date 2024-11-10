use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Input base
    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let base: f32 = input1.trim().parse().expect("Not a valid number");

    // Input height
    println!("Enter height: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let height: f32 = input2.trim().parse().expect("Not a valid number");

    // Calculate area if base and height are valid
    if base > 0.0 && height > 0.0 {
        let area: f32 = (base * height) / 2.0;
        println!("Area of the triangle: {}", area);
    } else {
        println!("Base and height must be greater than zero.");
    }
}
