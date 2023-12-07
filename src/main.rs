mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
use std::{fs::File, io::Read};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <day> <part>", args[0]);
        return;
    }

    let day = args[1].parse::<u8>().expect("Invalid day");

    let part = args[2].parse::<u8>().expect("Invalid part");

    if part != 1 && part != 2 {
        println!("Part must be 1 or 2");
        return;
    }

    match day {
        1 => day1::run(part, read_input()),
        2 => day2::run(part, read_input()),
        3 => day3::run(part, read_input()),
        4 => day4::run(part, read_input()),
        5 => day5::run(part, read_input()),
        6 => day6::run(part, read_input()),
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
