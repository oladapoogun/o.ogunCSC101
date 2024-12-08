use std::io;

fn main() {
    println!("1: Area of Trapezium: 0.5 * height * (base1 + base2)");
    println!("2: Area of Rhombus: 0.5 * diagonal1 * diagonal2");
    println!("3: Area of Parallelogram: base * altitude");
    println!("4: Area of Cube: 6 * side^2");
    println!("5: Volume of Cylinder: π * radius^2 * height");

    let selection = read_input("Enter the number of your choice (1-5): ");

    if selection == 1.0 {
        let height = read_input("Enter height: ");
        let base1 = read_input("Enter base1: ");
        let base2 = read_input("Enter base2: ");
        let area = 0.5 * height * (base1 + base2);
        println!("Area of Trapezium: {}", area);
    } else if selection == 2.0 {
        let diagonal1 = read_input("Enter diagonal1: ");
        let diagonal2 = read_input("Enter diagonal2: ");
        let area = 0.5 * diagonal1 * diagonal2;
        println!("Area of Rhombus: {}", area);
    } else if selection == 3.0 {
        let base = read_input("Enter base: ");
        let altitude = read_input("Enter altitude: ");
        let area = base * altitude;
        println!("Area of Parallelogram: {}", area);
    } else if selection == 4.0 {
        let side = read_input("Enter side length: ");
        let area = 6.0 * side * side;
        println!("Area of Cube: {}", area);
    } else if selection == 5.0 {
        let radius = read_input("Enter radius: ");
        let height = read_input("Enter height: ");
        let volume = std::f64::consts::PI * radius * radius * height;
        println!("Volume of Cylinder: {}", volume);
    } else {
        println!("Invalid input. Please select a number between 1 and 5.");
    }
}

fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
