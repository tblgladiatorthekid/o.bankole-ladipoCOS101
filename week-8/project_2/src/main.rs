fn main(){
   let mut name: Vec<String> = Vec::new();
   let mut exp: Vec<i32> = Vec::new();

    println!("--- ERNEST & YOUNG GLOBAL LTD CANDIDATE PORTAL---\nHere, you can enter the credentials of candidates and decide the best one.");

   println!("\nThere are {} candidates entered currently.",name.len());
   let mut input1 = String::new();
   std::io::stdin().read_line(&mut input1).expect("Failed to read input");
   let cnd : u32 = input1.trim().parse().expect("invalid input");

for count in 0..cnd {
    println!("\nCandidate {}:",count + 1);

   println!("\nEnter the candidate's name:\n");
   let mut can_in = String::new();
   std::io::stdin().read_line(&mut can_in).expect("Invalid input, enter their name");
   let candidate : String = can_in.trim().parse().expect("Invalid input");
   name.push(candidate);

   println!("\nEnter years of experience:\n");
   let mut exp_years = String::new();
   std::io::stdin().read_line(&mut exp_years).expect("Invalid input, expected integer");
   let years : i32 = exp_years.trim().parse().expect("Invalid integer");
   exp.push(years);
}

    println!("\nThe candidates entered include:\n");
    let  outcome = (name,exp);

    for (x,y) in outcome.0.iter().zip(outcome.1.iter()){
        println!("{} -- {} years of experience\n",x,y);
        
    }

    if let Some((best,max_value)) = outcome.0.iter().max().zip(outcome.1.iter().max()){
        println!("You should consider hiring: {} with {} years of experience.",best,max_value);
    }
    else{
        println!("\nNo candidates entered.");
    }
}
