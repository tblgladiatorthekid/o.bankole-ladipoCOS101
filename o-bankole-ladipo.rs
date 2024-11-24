use std::io;

fn main(){
	//Limit is 50
	let max_attempts = 50;
	let mut attempts = 0;
	while attempts < max_attempts {
	 println!("\nEnter your full name:");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Invalid input");
	let name = input1;

	println!("\nEnter your email:");
	let mut input5 = String::new();
	io::stdin().read_line(&mut input5).expect("Invalid input");
	let mail = input5;

	println!("\nEnter your department:");
	let mut input6 = String::new();
	io::stdin().read_line(&mut input6).expect("Invalid input");
	let department = input6;

	println!("\nEnter your state of origin:");
	let mut input7 = String::new();
	io::stdin().read_line(&mut input7).expect("Invalid input");
	let origin = input7;

	//Criteria to Vote
	println!("\nAre you a class rep?");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Invalid input");
	let rep: bool = match input2.trim().to_lowercase().as_str(){
		"yes" => true,
		"no"  => false,
		_=> {
			println!("Invalid input; put in either yes or no.");
			return;
		}
	}; 
	

	println!("\nAre you in 100 Level?");
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("Invalid input");
	let lvl: bool = match input3.trim().to_lowercase().as_str(){
		"yes" => true,
		"no"  => false,
			_=> {
			println!("Invalid input; put in either yes or no.");
			return;
		}

	}; 
	

	println!("\nWhat is your CGPA?");
	let mut input4 = String::new();
	io::stdin().read_line(&mut input4).expect("Expected integer");
	let gpa : f32 = input4.trim().parse().expect("Invalid, expected integer");

	if gpa > 5.0 {
		println!("Invalid input; CGPA is between 0.1 and 5.0");
	};

	//Can vote
	if rep == true && lvl == false && gpa > 4.0 {
		println!("{}\n{}\n{}\n{}",name,mail,department,origin);
		println!("\nYou are eligible to vote.");
}	//Can not vote
	else {
		println!("Sorry, you are not eligible to vote.");
}
	attempts+= 1; 
};
}

//use std::io;

//fn main (){
	//Limit is 100
	//let max_attempts = 100;
	//let mut attempts = 0;
	//while attempts < max_attempts{
	//println!("\nEnter your name:");
	//let mut input1 = String::new();
	//io::stdin().read_line(&mut input1).expect("Invalid input");
	//let name = input1;

	//println!("\nEnter the number of papers you have published:");
	//let mut input2 = String::new();
	//io::stdin().read_line(&mut input2).expect("Invalid input");
	//let paper: i32 = input2.trim().parse().expect("Invalid input");


	//if paper >= 3 && paper <=5{
		//println!("You will receive an incentive of N500,000.00");
	//}

	//else if paper > 5 && paper < 10{
		//println!("You will receive an incentive of N800,000.00");
	//}

	//else if paper >= 10{
		//println!("You will receive an incentive of N1,000,000.00");
	//}

	//else if paper < 3{
		//println!("You will receive an incentive of N100,000.00");
	//}

	//attempts+= 1; 
//};

//}
