mod day1;
mod day2;
mod day3;
use std::{fs::File, io::Read};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <day> <part>", args[0]);
        return;
    }

    let day = args[1].parse::<u8>().unwrap_or_else(|_| {
        println!("Invalid day: {}", args[1]);
        std::process::exit(1);
    });

    let part = args[2].parse::<u8>().unwrap_or_else(|_| {
        println!("Invalid part: {}", args[2]);
        std::process::exit(1);
    });

    match day {
        1 => day1::run(part, read_input()),
        2 => day2::run(part, read_input()),
        3 => day3::run(part, read_input()),
        _ => {
            println!("Invalid day: {}", day);
            std::process::exit(1);
        }
    }
}

fn read_input() -> String {
    let mut file = File::open("input.txt").unwrap_or_else(|_| {
        println!("Failed to open input.txt");
        std::process::exit(1);
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_| {
        println!("Failed to read input.txt");
        std::process::exit(1);
    });
    return contents;
}
