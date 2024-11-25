use std::io;

fn equation(h:f32, l:f32, a:f32) {
   let  t = h/2.0 * (l + a);
   let  r = 0.5 * l * h * a.powf(0.0);
   let  p: f32 = l * a * h.powf(0.0);
   let  c = 6.0 * l.powf(2.0) *h.powf(0.0) * a.powf(0.0);
   let  v = 22.0/7.0 * l.powf(2.0) * h * a.powf(0.0);



let formulae = [("T",t),("R",r),("P",p),("C",c),("V",v)];



    println!("\nPlease select an equation: [Choose either T, R, P, C or V]");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let express =input1.trim().to_uppercase();

    let value = formulae.iter().find(|&&(code, _)| code == express);
    if let Some(&(_,math)) = value {
        println!("Your answer is: {}",math);
        return;
}
    

}


fn main() {
    println!("Hello user; this program is used to solve for the following:
        T = Area of trapezium
        R = Area of rhombus
        P = Area of parallelogram
        C = Area of Cube
        V = Volume of Cylinder");
    println!("\nYou are allowed to input only three variables: h, l and a.");

     

    println!("\nEnter your dimension for h:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input, expected integer");
    let h: f32 = input2.trim().parse().expect("Invalid input, expected integer");

     println!("\nEnter your dimension for l:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Invalid input, expected integer");
    let l: f32 = input3.trim().parse().expect("Invalid input, expected integer");

     println!("\nEnter your dimension for a:");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Invalid input, expected integer");
    let a: f32 = input4.trim().parse().expect("Invalid input, expected integer");

    equation(h, l, a);

    }
