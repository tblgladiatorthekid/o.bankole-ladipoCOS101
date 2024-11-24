use std::io;

fn main (){
	//Limit is 100
	let max_attempts = 100;
	let mut attempts = 0;
	while attempts < max_attempts{
	println!("\nEnter your name:");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Invalid input");
	let name = input1;

	println!("\nEnter the number of papers you have published:");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Invalid input");
	let paper: i32 = input2.trim().parse().expect("Invalid input");


	if paper >= 3 && paper <=5{
		println!("You will receive an incentive of N500,000.00");
	}

	else if paper > 5 && paper < 10{
		println!("You will receive an incentive of N800,000.00");
	}

	else if paper >= 10{
		println!("You will receive an incentive of N1,000,000.00");
	}

	else if paper < 3{
		println!("You will receive an incentive of N100,000.00");
	}

	attempts+= 1; 
};

}