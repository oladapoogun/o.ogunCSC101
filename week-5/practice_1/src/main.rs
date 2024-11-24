fn main() {
    let name = "Aisha lawal";
    let uni:&str="Pan-Atlantic University";
    let addr:&str="Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name:{}",name);
    println!("University: {}, \nAdresss:{}",uni,addr);

    let department:&'static str ="Computer Science";
    let school:&'static str="School of science and Technology";
    println!("Department:{},\nSchool:{}",department,school)
}
