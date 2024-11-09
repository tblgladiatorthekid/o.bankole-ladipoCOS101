use std::io;

fn main() {
    println!("Enter your age");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let age: i32 = input1.trim().parse().expect("Not valid integer");

    println!("Enter your experience of employment (yes or no): ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let exp: bool = match input2.trim().to_lowercase().as_str() {
        "yes" => true,
        "no" => false,
        _ => {
            println!("Invalid input. Please enter 'yes' or 'no'.");
            return;

        } 
          };




    if age >= 40 && exp == true {
        println!("You will receive an incentive of N1,560,000");
    }

    else if 30 <= age && age < 40 && exp == true {
        println!("You will receive an incentive of N1,480,000");
    }

    else if age < 28 && exp == true {
        println!("You will receive an incentive of N1,300,000");
    }

    else if exp == false {
        println!("You will receive an incentive of N100,000");
    }

}
