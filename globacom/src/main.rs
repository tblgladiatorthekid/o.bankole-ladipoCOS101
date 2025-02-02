use std::fs;
use std::io;

fn input(que: &str) -> String{
    println!("{}",que);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_file(file_name: &str) {
match fs::read(file_name) {
        Ok(data) => {
            let content = String::from_utf8_lossy(&data);
            println!("File contents:\n{}", content);
        }
        Err(e) => println!("Error reading file '{}': {}", file_name, e),
    }
}


fn main() {
    let user = vec!["A","M","E","C","V"];
    println!("=== GLOBACOM DATABASE ===");
    loop { 
        let user_type = input("\nGood day user. Please make known your status: 
        A => Administrator
        M => Project Manager
        E => Employee
        C => Customer
        V => Vendor").to_uppercase();

    if user.contains(&user_type.as_str()){ 
        match user_type.as_str(){
                "A" => read_file("globacom.sql"),
                "M" => read_file("project.sql"),
                "E" => read_file("staff.sql"),
                "C" => read_file("customer_table.sql"),
                "V" => read_file("dataplan_table.sql"),
                _ => unreachable!(),
            }
    }
     else {
        println!("Invalid input, please look at options");
        continue;
        }

    let choice = input("\n you want to continue? Y (yes) or N (no)").to_uppercase();

    if choice == "N" {
        break;
    }

    else if choice != "Y" {
            println!("Invalid choice. Exiting...");
            break;
}

}

}
