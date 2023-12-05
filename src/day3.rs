use crate::read_input;

pub fn print_solution() {
    static DAY: &str = "day3_ex";
    let input = read_input(DAY);
    let lines = input.split("\n").collect::<Vec<&str>>();

    print!("\n--- {} ---\nAnswere to part one: \n", DAY,);
    println!("{}", part_one(lines.clone()));
    print!("\nAnswere to part two: ");
    println!("{}\n---", part_two(lines.clone()))
}

fn part_one(lines: Vec<&str>) -> String {
    let matrix: Vec<Vec<char>> = create_matrix(lines);

    for (i, row) in matrix.iter().enumerate() {
        for (col, symbol) in row.iter().enumerate() {
            if symbol.is_numeric() {
                let len = get_number_len(&row, col);
                println!("Found number of length {} in row {}", len, i)
            }
        }
    }
    println!("\n{}", matrix[0][1]);
    return "###NOT DONE###".to_string();
}

fn part_two(lines: Vec<&str>) -> String {
    return "###NOT DONE###".to_string();
}

fn create_matrix(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        matrix.push(line.chars().collect());
    }
    return matrix;
}

fn get_number_len(row: &Vec<char>, col: usize) -> i32 {
    let mut length: i32 = 0;
    for index in col..row.len() {
        if row[index].is_numeric() {
            println!("it's numeric see {}", row[index]);
            length += 1;
        } else {
            break;
        }
    }
    return length;
}
