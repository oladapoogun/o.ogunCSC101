use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Input name
    println!("Enter Your name: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");

    // Input age
    println!("Enter Your age: ");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let age: i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 18 {
        println!("Welcome to the party {}", input1.trim());
    } else {
        println!("oops, you are not of age to enter the party {}", input1.trim());
    }
}
