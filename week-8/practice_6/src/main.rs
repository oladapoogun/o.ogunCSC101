fn main() {
    // Create two vectors
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x = vec![1, 5, 7, 9, 11, 13];

    // Use a for loop to add elements of the vectors
    for index in 0..6 {
        let sum = v[index] + x[index];
        println!("{} {}", index, sum);
    }
}