use crate::read_input;

pub fn print_solution() {
    static DAY: &str = "day2";
    let input = read_input(DAY);
    let lines = input.split("\n").collect::<Vec<&str>>();

    print!("\n--- {} ---\nAnswere to part one: ", DAY,);
    println!("{}", part_one(lines.clone()));
    print!("\nAnswere to part two: ");
    println!("{}\n---", part_two(lines.clone()))
}

fn part_one(lines: Vec<&str>) -> String {
    static RED_MAX: i8 = 12;
    static GREEN_MAX: i8 = 13;
    static BLUE_MAX: i8 = 14;
    // for line in lines {
    //     let parts = line.split(":");
    //     for part in parts {
    //         println!("{}", part);
    //     }
    // }
    let mut id_sum: i32 = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let mut did_break = false;

        for round in parts[1].split("; ") {
            let cubes: Vec<&str> = round.split(", ").collect();
            for cube in cubes {
                let result: Vec<&str> = cube.split_whitespace().collect();
                match result[1] {
                    "red" => {
                        if result[0].parse::<i8>().unwrap() > RED_MAX {
                            // println!("Broke on too many red");
                            did_break = true;
                            break;
                        }
                    }
                    "green" => {
                        if result[0].parse::<i8>().unwrap() > GREEN_MAX {
                            // println!("Broke on too many green");
                            did_break = true;
                            break;
                        }
                    }
                    "blue" => {
                        if result[0].parse::<i8>().unwrap() > BLUE_MAX {
                            // println!("Broke on too many blue");
                            did_break = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if did_break {
                break;
            }
        }
        if did_break {
            continue;
        } else {
            let game: Vec<&str> = parts[0].split_whitespace().collect();
            // println!("Adding game {} to the sum", game[1].parse::<i32>().unwrap());
            id_sum += game[1].parse::<i32>().unwrap();
        }
    }
    // println!("{}", id_sum);
    return format!("{}", id_sum);
}

fn part_two(lines: Vec<&str>) -> String {
    let mut power_sum: i32 = 0;

    for line in lines {
        let game: Vec<&str> = line.split(": ").collect();

        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        for round in game[1].split("; ") {
            let cubes: Vec<&str> = round.split(", ").collect();
            for cube in cubes {
                let result: Vec<&str> = cube.split_whitespace().collect();
                let number: i32 = result[0].parse::<i32>().unwrap();
                match result[1] {
                    "red" => {
                        if number > red_cubes {
                            red_cubes = number;
                        }
                    }
                    "green" => {
                        if number > green_cubes {
                            green_cubes = number;
                        }
                    }
                    "blue" => {
                        if number > blue_cubes {
                            blue_cubes = number;
                        }
                    }
                    _ => {}
                }
            }
        }
        power_sum += red_cubes * green_cubes * blue_cubes;
    }
    println!("{}", power_sum);
    return "###NOT DONE###".to_string();
}
