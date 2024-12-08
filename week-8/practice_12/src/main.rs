fn main() {
   
   let mut colours = ["red","green","yellow","white"];

   println!("\nOriginal array = {:?}", colours);

   let sliced_colours = &mut colours[1..3];
   println!("First slice = {:?}", sliced_colours);

   sliced_colours[1] = "purple";

   println!("Changed slice = {:?}", sliced_colours);
}
