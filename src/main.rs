use std::env;
use std::error::Error;

pub mod parser {
    pub mod file;
}

mod solutions {
    pub mod day_1 {
        pub mod calorie_counting;
    }
    pub mod day_2 {
        pub mod rock_paper_scissors;
    }
    pub mod day_3 {
        pub mod rucksack_reorganization;
    }
    pub mod day_4 {
        pub mod camp_cleanup;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Example: {} 01", args[0]);
        return Ok(());
    }
    match args[1].as_str() {
        "1" => solutions::day_1::calorie_counting::run(),
        "2" => solutions::day_2::rock_paper_scissors::run(),
        "3" => solutions::day_3::rucksack_reorganization::run(),
        "4" => solutions::day_4::camp_cleanup::run(),
        _ => Ok(()),
    }
}
