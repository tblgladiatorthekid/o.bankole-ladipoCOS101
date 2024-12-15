use std::io::Write;

fn main() {
    let title = "==NIGERIAN BREWERIES PLC==";
    let mut lager : Vec<String> = Vec::new();
    let mut stout : Vec<String> = Vec::new();
    let mut non_alc: Vec<String> = Vec::new();

    println!("Welcome.Here you can enter what the type and the number of a specific type of drink.\n");
    println!("No. of LAGER entered: {}",lager.len());
    println!("No. of STOUT entered: {}",stout.len());
    println!("No. of NON-ALCOHOLIC entered: {}",non_alc.len());

    let  mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Invalid input, expected string");
    let new_drink1 : u32 = input.trim().parse().expect("Invalid input");

    let  mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Invalid input, expected string");
    let new_drink2 : u32 = input2.trim().parse().expect("Invalid input");

    let  mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Invalid input, expected string");
    let new_drink3 : u32 = input3.trim().parse().expect("Invalid input");

    let mut file = std::fs::File::create("lager_table.txt").expect("create failed");
    file.write_all(title.as_bytes()).expect("write failed");

    for x in 0..new_drink1{
            println!("\nYou have entered {} drink(s) for lager.",x + 1);
            println!("\nEnter the name of your lager:\n");
            let mut input4 = String::new();
            std::io::stdin().read_line(&mut input4).expect("Invalid input");
            let drk_choice1: String = input4.trim().parse().expect("Invalid input");
            lager.push(drk_choice1);
        }

    for y in 0..new_drink2{
            println!("\nYou have entered {} drink(s) for stout.",y + 1);
            println!("Enter the name of your stout:\n");
            let mut input5 = String::new();
            std::io::stdin().read_line(&mut input5).expect("Invalid input");
            let drk_choice2: String = input5.trim().parse().expect("Invalid input");
            stout.push(drk_choice2);
        }  
 
    for z in 0..new_drink3{
            println!("\nYou have entered {} drink(s) for non-alcoholic.",z + 1);
            println!("Enter the name of your drink:");
            let mut input6 = String::new();
            std::io::stdin().read_line(&mut input6).expect("Invalid input");
            let drk_choice3: String = input6.trim().parse().expect("Invalid input");
            non_alc.push(drk_choice3);
        }

    let brew_title = "\n|LAGER| |STOUT| |NON-ALCOHOLIC|\n";
    file.write_all(brew_title.as_bytes()).expect("write failed");

    let  table: Vec<Vec<String>> = vec![lager,stout,non_alc];
    let max_rows = table.iter().map(|data| data.len()).max().unwrap_or(0);
    for i in 0..max_rows {
        let row: Vec<String> = table.iter()
        .map(|data| data.get(i).unwrap_or(&String::from("")).to_string())
        .collect();
         file.write_all(format!("|{:12}|{:12}|{:14}|\n", row[0], row[1], row[2]).as_bytes()).expect("write failed");
    }
    println!("\nData written to file. ")
}
