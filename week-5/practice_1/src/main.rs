fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";//used to view string you do not plan to modify
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}",name);
    println!("University: {}, \nAddress: {}",uni,addr);

    let department:&'static str = "Computer Science";//static string is automatic
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school);
}
