use crate::read_input;

pub fn print_solution() {
    static DAY: &str = "day3";
    let input = read_input(DAY);
    let lines = input.split("\n").collect::<Vec<&str>>();

    print!("\n--- {} ---\nAnswere to part one: \n", DAY,);
    println!("{}", part_one(lines.clone()));
    print!("\nAnswere to part two: ");
    println!("{}\n---", part_two(lines.clone()))
}

fn part_one(lines: Vec<&str>) -> String {
    let matrix: Vec<Vec<char>> = create_matrix(lines);
    let mut total_sum = 0;
    println!("Matrix size {} x {}", matrix.len(), matrix[0].len());

    for row in 0..matrix.len() {
        let mut col = 0;
        while col < matrix[row].len() {
            if matrix[row][col].is_numeric() {
                let len = get_number_len(&matrix[row], col);
                // println!("Found number of length {} in row {}", len, row);
                if check_neighbors(&matrix, row, col, len) {
                    let mut s = String::new();
                    for i in 0..len {
                        s.push(matrix[row][col + i])
                    }
                    // println!("Found number {} in row {}", s.parse::<i32>().unwrap(), row);
                    total_sum += s.parse::<i32>().unwrap();
                }
                col += len;
            }
            col += 1;
        }
    }
    return format!("{}", total_sum);
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

fn get_number_len(row: &Vec<char>, col: usize) -> usize {
    let mut length: usize = 0;
    for index in col..row.len() {
        if row[index].is_numeric() {
            // println!("it's numeric see {}", row[index]);
            length += 1;
        } else {
            break;
        }
    }
    return length;
}

fn check_neighbors(matrix: &Vec<Vec<char>>, row: usize, col: usize, len: usize) -> bool {
    let col_min = if col == 0 { 0 } else { col - 1 };
    let col_max = if col + len == matrix[row].len() {
        col + len
    } else {
        col + len + 1
    };
    let row_min = if row == 0 { 0 } else { row - 1 };
    let row_max = if row + 1 == matrix.len() {
        row + 1
    } else {
        row + 2
    };
    println!(
        "row {}, max {}, min {}\ncol {}, max {}, min {}",
        row, row_max, row_min, col, col_max, col_min
    );
    // println!("Checking row {} col {}", row, col);
    for i in row_min..row_max {
        for j in col_min..col_max {
            // print!("{}", matrix[i][j]);
            if matrix[i][j] != '.' && !matrix[i][j].is_numeric() {
                return true;
            }
        }
    }
    return false;
}
