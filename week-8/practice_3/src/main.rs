//Method to print the get value
fn value(n:Option<&char>)
{
    println!("Element of vector {:?}",n );


}
fn main() {

    let v=vec!['r','u','s','t','a','c','t','a','n'];

    let mut input1=String::new();
    println!("\nEnter an index value (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    //index is the non negative value which is smaller than  the size of the vector
    let index:usize=input1.trim().parse().expect("invalid input");

    // getting value at given index value
    let ch: Option<&char> = v.get(index);
    value(ch);

}
