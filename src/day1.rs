use crate::read_input;
use regex::Regex;

pub fn print_solution() {
    let input = read_input("day1");
    let lines = input.split("\n").collect::<Vec<&str>>();

    println!(
        "\n--- Day1 ---\n---\nAnswere to part one: {}\n---",
        part_one(lines.clone())
    );
    println!("Answere to part two: {}\n---", part_two(lines.clone()));
}

fn part_one(lines: Vec<&str>) -> String {
    let mut elements: Vec<i32> = vec![];

    for line in lines {
        let mut coordinate = String::from("");
        for c in line.chars() {
            if c.is_digit(10) {
                coordinate.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                coordinate.push(c);
                break;
            }
        }
        elements.push(coordinate.parse::<i32>().unwrap_or(0))
    }

    let sum: i32 = elements.iter().sum();

    return sum.to_string();
}

fn part_two(lines: Vec<&str>) -> String {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|zero|[0-9])").unwrap();
    let re_rev = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez|[0-9])").unwrap();

    let mut elements: Vec<i32> = vec![];

    for line in lines {
        let mut vec: Vec<i32> = Vec::new();

        for caps in re.captures_iter(line) {
            let val = caps.get(0).unwrap().as_str();
            if val.chars().all(|c| c.is_digit(10)) {
                vec.push(val.parse::<i32>().unwrap_or(0));
                break;
            }
            if to_number(val) != None {
                vec.push(to_number(val).unwrap_or(0));
                break;
            }
        }
        let line_rev = line.chars().rev().collect::<String>();
        for caps in re_rev.captures_iter(&line_rev) {
            let val: &str = &caps
                .get(0)
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>();
            if val.chars().all(|c| c.is_digit(10)) {
                vec.push(val.parse::<i32>().unwrap_or(0));
                break;
            }
            if to_number(val) != None {
                vec.push(to_number(val).unwrap_or(0));
                break;
            }
        }

        elements.push(vec.first().unwrap() * 10 + vec.last().unwrap());
    }

    let sum: i32 = elements.iter().sum();

    return sum.to_string();
}

fn to_number(number: &str) -> Option<i32> {
    match number {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
