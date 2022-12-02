use std::env;

pub mod parser {
    pub mod file;
}

mod solutions {
    pub mod day_1 {
        pub mod elves_gathering_calories;
    }
    pub mod day_2 {
        pub mod rock_paper_scissors;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Example: {} 01", args[0]);
        return;
    }
    match args[1].as_str() {
        "1" => solutions::day_1::elves_gathering_calories::run(),
        "2" => solutions::day_2::rock_paper_scissors::run(),
        _ => println!("day not found"),
    }
}
