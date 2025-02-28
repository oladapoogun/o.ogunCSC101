// The iter() funnction fetches values of all elements in an array
fn main() {
    let arr: [i32; 4] = [10, 20, 30, 40];

    println!("Array is: {:?}", arr);
    println!("Array size is: {}", arr.len());

    for val in arr.iter() {
        println!("Value is: {}", val); 

    }
}
