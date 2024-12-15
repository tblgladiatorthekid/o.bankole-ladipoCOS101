use std::io::Write;
use std::io::Read;
fn main() {
   let title = "\n===EFCC DATABASE===\n";
   let name = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu",
   "Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];

   let min = vec!["Internal Affairs","Justice",
   "Defense","Power & Steel","Petroleum"];

   let zone = vec!["South West","North East","South South","South West","South East"];

   let table: [[&str; 3]; 6] = [
                ["NAME","MINISTRY","GEOPOLITICAL ZONE"],
                [name[0],min[0],zone[0]],
                [name[1],min[1],zone[1]],
                [name[2],min[2],zone[2]],
                [name[3],min[3],zone[3]],
                [name[4],min[4],zone[4]],
   ];
   let mut file = std::fs::File::create("efcc.txt").expect("create failed");  
    file.write_all(title.as_bytes()).expect("write failed");

   for row in &table {
    let line = row.join("|");
    file.write_all(format!("\n|{}|\n",line).as_bytes()).expect("write failed");
   }
   println!("\nData written to file.");
   let mut file_2 = std::fs::File::open("efcc.txt").unwrap();
    let mut data = String::new();
    file_2.read_to_string(&mut data).unwrap();
    print!("{}",data);
}
