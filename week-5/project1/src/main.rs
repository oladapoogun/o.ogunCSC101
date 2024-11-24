use std::io;

fn main() {
    // Define prices for the food items
    let p_price = 3200; // Poundo Yam/Edinkaiko Soup
    let f_price = 3000; // Fried Rice & Chicken
    let a_price = 2500; // Amala & Ewedu Soup
    let e_price = 2000; // Eba & Egusi Soup
    let w_price = 2500; // White Rice & Stew

    // Display the menu
    println!("Welcome to the Food Ordering System!");
    println!("Please select the food and quantity:");
    println!("P - Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F - Fried Rice & Chicken - N3,000");
    println!("A - Amala & Ewedu Soup - N2,500");
    println!("E - Eba & Egusi Soup - N2,000");
    println!("W - White Rice & Stew - N2,500");

    // Read food choice
    let mut food_choice = String::new();
    println!("\nEnter the food code (P, F, A, E, W):");
    io::stdin().read_line(&mut food_choice).expect("Failed to read line");
    let food_choice = food_choice.trim().to_uppercase(); // Convert input to uppercase

    // Read quantity with a simple instruction
    let mut quantity_input = String::new();
    println!("\nEnter the quantity (e.g., 1, 2, 3):");
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");

    // Try to convert quantity to a number
    let quantity: u32 = quantity_input.trim().parse().unwrap_or(0); // If not a number, use 0

    if quantity == 0 {
        println!("Please enter a valid number for quantity.");
        return; // Exit if quantity is invalid
    }

    // Calculate the total price
    let mut total_price = 0;
    
    if food_choice == "P" {
        total_price = p_price * quantity;
    } else if food_choice == "F" {
        total_price = f_price * quantity;
    } else if food_choice == "A" {
        total_price = a_price * quantity;
    } else if food_choice == "E" {
        total_price = e_price * quantity;
    } else if food_choice == "W" {
        total_price = w_price * quantity;
    } else {
        println!("Invalid food choice. Please choose a valid option.");
        return; // Exit if food choice is invalid
    }

    // Check if the total price is greater than N10,000 to apply a discount
    if total_price > 10000 {
        let discount = (total_price as f64 * 0.10) as u32; // 10% discount
        total_price -= discount;
        println!("\nA discount of 10% has been applied.");
    }

    // Display the final total price
    println!("\nTotal price for your order is: N{}", total_price);
}
