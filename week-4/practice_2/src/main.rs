use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    // Input first edge of triangle
    println!("Enter first edge of the triangle: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    // Input second edge of triangle
    println!("Enter second edge of the triangle: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    // Input third edge of triangle
    println!("Enter third edge of the triangle: ");
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    // Calculate semi-perimeter
    let s: f32 = (a + b + c) / 2.0;

    // Calculate area using Heron's formula
    let mut area: f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    // Output the area
    println!("Area of the triangle: {}", area);
}
