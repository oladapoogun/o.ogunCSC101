fn main() {
    let a = 20;
    let b = 30;

    // Check if both a and b are greater than 10
    if (a > 10) && (b > 10) {
        println!("Both a and b are greater than 10");
    }

    let c = 0;
    let d = 30;

    // Check if either c or d is greater than 10
    if (c > 10) || (d > 10) {
        println!("Either c or d is greater than 10");
    }

    let is_elder = false;

    // Check if is_elder is false
    if !is_elder {
        println!("Not Elder");
    }
}
