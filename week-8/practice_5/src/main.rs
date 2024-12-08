fn main() {
   let mut city: Vec<String> = Vec::new();

   println!("The City vector has element {}",city.len());

   let mut input1 = String::new();
   std::io::stdin().read_line(&mut input1).expect("Failed to read input");
   let city_num : i32 = input1.trim().parse().expect("invalid input");

   for count in 0..city_num {
    let mut input2 = String::new();
    println!("Enter city {}",count+1);
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let new_city :String = input2.trim().parse().expect("invalid input");
    city.push(new_city);
   }

print!("The preferred cities are:\n");
let mut count=1;

for i in city{
    println!("{} {}", count, i);
    count+=1;
}
}
