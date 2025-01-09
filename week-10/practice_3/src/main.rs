fn main() {

    let v = vec![20, 40,60, 80];

    display(v);
}

fn display(v:Vec<i32>) ->Vec<i32> {
    println!("The value of v is {:?}",v);
    return v;
}
