use std::env;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).unwrap();

    println!("Running {day}");
    match day.as_str() {
        "day01" => day01::run(),
        "day02" => day02::run(),
        _ => println!("No day selected"),
    }
}
