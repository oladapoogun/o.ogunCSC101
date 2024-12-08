fn main() {
    // Vectors as per roles and their corresponding levels
    let public_servant_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let office_admin_roles = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic_roles = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer_roles = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher_roles = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    // Prompt user for input
    println!("Enter your role (Office Administrator, Academic, Lawyer, or Teacher): ");
    let mut user_role = String::new();
    std::io::stdin().read_line(&mut user_role).expect("Failed to read input");
    let user_role = user_role.trim();

    println!("Enter your years of experience: ");
    let mut user_experience = String::new();
    std::io::stdin().read_line(&mut user_experience).expect("Failed to read input");
    let years: usize = user_experience.trim().parse().expect("Invalid input");

    // Determine index range based on years of experience
    let index = if years <= 2 {
        0
    } else if years <= 5 {
        1
    } else if years <= 8 {
        2
    } else if years <= 10 {
        3
    } else if years <= 13 {
        4
    } else {
        5
    };

    // Logic to handle role type and determine the corresponding position
    if user_role == "Office Administrator" {
        println!("Your Public Service APS Level is: {}", public_servant_levels[index]);
        println!("Your position is: {}", office_admin_roles[index]);
    } else if user_role == "Academic" {
        println!("Your Public Service APS Level is: {}", public_servant_levels[index]);
        println!("Your position is: {}", academic_roles[index]);
    } else if user_role == "Lawyer" {
        println!("Your Public Service APS Level is: {}", public_servant_levels[index]);
        println!("Your position is: {}", lawyer_roles[index]);
    } else if user_role == "Teacher" {
        println!("Your Public Service APS Level is: {}", public_servant_levels[index]);
        println!("Your position is: {}", teacher_roles[index]);
    } else {
        println!("Invalid role type entered.");
    }
}
