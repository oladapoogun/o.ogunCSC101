fn main() {
    // Name vector
    let name = vec!["Marry", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
    println!("Hello, world!");

    // Age vector
    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];

    println!("\nAge allocation:\n");

    // Loop to iterate through elements in vectors
    for i in 0..age.len() {
        println!("{} is {} years old", name[i], age[i]);
    }
}
