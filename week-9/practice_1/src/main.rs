use std::io::write;

fn main() {
    let announce="Week 9-Rust File input %Output\n";
    let dept="Department of computer science";

    let mut file=std::fs::File::create("data.txt").expect("Create Failed");
    file.write_all("Welcome to Rust programming\n";
        .as_bytes()).expect("Write failed");
    file.write_all(announce.as_bytes()).expect("Write failed");
    file.write_all(dept.as_bytes()).expect("Write failed");
    println!("\n Datat written to file");
}