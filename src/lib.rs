use std::fs;

pub mod day1;

pub fn read_input(day: &str) -> String {
    let input = fs::read_to_string(format!("input/{day}.txt")).expect("This should work");
    return input;
}
