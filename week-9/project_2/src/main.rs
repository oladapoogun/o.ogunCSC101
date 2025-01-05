use std::fs::File;
use std::io::{self, Write};

fn main() {
    // Define the student data
    let students = vec![
        ("Oluchi Mordi", "ACC1021111", "Accounting", 400),
        ("Adams Aliyu", "ECO1010101", "Economics", 300),
        ("Shania Bolade", "CSC1032882", "Computer Science", 200),
        ("Adekunle Gold", "EEE1020202", "Electrical", 500),
        ("Blanca Edemon", "MEE1020201", "Mechanical", 100),
    ];

    // Display student details
    println!("PAU SMIS\n");
    println!("Student Name\tMatric. Number\tDepartment\tLevel");
    for student in &students {
        println!(
            "{}\t{}\t{}\t{}",
            student.0, student.1, student.2, student.3
        );
    }

    // Save to file
    let file_path = "PAU_SMIS.txt";
    let mut file = File::create(file_path).expect("Could not create file");

    writeln!(file, "PAU SMIS").unwrap();
    writeln!(file, "Student Name\tMatric. Number\tDepartment\tLevel").unwrap();
    for student in students {
        writeln!(
            file,
            "{}\t{}\t{}\t{}",
            student.0, student.1, student.2, student.3
        )
        .unwrap();
    }

    println!("\nStudent details have been saved to {}", file_path);
}
