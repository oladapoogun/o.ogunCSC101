use std::io; // Import standard input module

fn main() {
    // Display a welcome message
    println!("Welcome to the EY Nigeria Programming Experience Validator System!");
    println!("You can input the names and years of experience of programmers to find the one with the most experience.\n");

    let mut programmers = Vec::new(); // Vector to store user input dynamically

    // Ask the user how many programmers they want to input
    let mut input = String::new();
    println!("How many programmers do you want to input?");
    io::stdin()
        .read_line(&mut input) // Read user input
        .expect("Failed to read input");
    let num_programmers: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // Loop to get user data dynamically
    for _ in 0..num_programmers {
        let mut name = String::new();
        let mut experience_input = String::new();

        // Get name input
        println!("Enter programmer's name:");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        // Get years of experience input
        println!("Enter their years of programming experience:");
        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read input");

        // Parse experience as integer
        let experience: u32 = match experience_input.trim().parse() {
            Ok(exp) => exp,
            Err(_) => {
                println!("Invalid input for experience. Please enter a number.");
                return;
            }
        };

        // Push the data into the vector
        programmers.push((name.trim().to_string(), experience));
    }

    // Ensure there are programmers to evaluate
    if programmers.is_empty() {
        println!("No programmers were entered.");
        return;
    }

    // Initialize with the first programmer's data
    let mut max_experience = programmers[0].1;
    let mut max_person = &programmers[0].0;

    // Loop through the vector to find the programmer with the highest experience
    for programmer in &programmers {
        if programmer.1 > max_experience {
            max_experience = programmer.1;
            max_person = &programmer.0;
        }
    }

    // Output the result
    println!(
        "The person with the highest programming experience is: {} with {} years.",
        max_person, max_experience
    );
}
