use std::io;

fn main() {
    println!("Enter a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not valid integer");

    println!("Enter b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not valid integer");

    println!("Enter c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not valid integer");

    let d: f32 = b.powf(2.0) - 4.0 * a * c;

    if d > 0.0 {
        println!("The discriminant is {}", d);
        println!("The equation has two roots");
    }

    else if d < 0.0 {
        println!("The discriminant is {}", d);
        println!("The equation has no real roots");
    }

    else if d == 0.0 {
        println!("The discriminant is {}", d);
        println!("The equation has one root");
    }




    
    


}