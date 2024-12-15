use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("PAU SMIS.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    print!("{}",data);
}
