fn main() {
    let jobs = vec![
                 vec!["Intern","Paralegal","Placement"],
                 vec!["Administrator","Research Assistant","Junior Associate","Classroom Teacher"],
                 vec!["Senior Administrator","PhD Candidate","Associate","Senior Teacher"],            
                 vec!["Office Manager","Post-Doc Researcher","Senior Associate 1-2","Leading Teacher"],
                 vec!["Director","Senior Lecturer","Senior Associate 3-4","Deputy Principal"],
                vec!["CEO","Dean","Partner","Principal"],
         ];

    let exp_range = vec![(1,2),(3,5),(5,8),(8,10),(10,13),(13,20)];

    let title = vec!["APS 1-2","APS 3-5","APS 5-8",
                    "EL1 8-10","EL2 10-13","SES"];

    print!("-----PUBLIC SERVICE APS LEVEL CHECKER-----");

    println!("\nWelcome.");

    println!("\nEnter your name:\n");
    let mut input4 = String::new();
    std::io::stdin().read_line(&mut input4).expect("Invalid input");
    let name: String = input4.trim().parse().expect("Invalid input");

    println!("\nEnter your profession:\n");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Invalid input");
    let prof : &str = input2.trim();

    let jobs_index = jobs.iter().position(|job_lvl|job_lvl.contains(&&prof));
    
    if let Some(index) = jobs_index{
        println!("\nEnter your your years of experience:\n");
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("Invalid input");
        let years: i32 = match input3.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Invalid input");
                return;
            }
        };
    

    let (min_year , max_year) = exp_range[index];
    if years >= min_year && years <= max_year {
        let titles = title[index];
        println!("\nHello {}. As a {} with {} years of working experience, your position is {}.",name,prof,years,titles);

    }

    else {
        println!("\nSorry, {}. Your work experience does not align with the APS system.",name);
    }        
}

    else {
        println!("Unfortunately {}, your job is not listed in our APS system.",name);
    }

}
