use crate::read_input;

pub fn print_solution() {
    static DAY: &str = "day3";
    let input = read_input(DAY);
    let lines = input.split("\n").collect::<Vec<&str>>();

    print!("\n--- {} ---\nAnswere to part one: ", DAY,);
    println!("{}", part_one(lines.clone()));
    print!("\nAnswere to part two: ");
    println!("{}\n---", part_two(lines.clone()))
}

fn part_one(lines: Vec<&str>) -> String {
    let matrix: Vec<Vec<char>> = create_matrix(lines);
    let mut total_sum = 0;

    for row in 0..matrix.len() {
        let mut col = 0;
        while col < matrix[row].len() {
            if matrix[row][col].is_numeric() {
                let len = get_number_len(&matrix[row], col);
                if check_neighbors(&matrix, row, col, len) {
                    let mut s = String::new();
                    for i in 0..len {
                        s.push(matrix[row][col + i])
                    }
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
    let matrix: Vec<Vec<char>> = create_matrix(lines);
    let mut total_sum = 0;

    for row in 0..matrix.len() {
        let mut col = 0;
        while col < matrix[row].len() {
            if matrix[row][col] == '*' {
                total_sum += check_surounding_number(&matrix, row, col);
            }
            col += 1;
        }
    }
    return format!("{}", total_sum);
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
    // println!(
    //     "row {}, max {}, min {}\ncol {}, max {}, min {}",
    //     row, row_max, row_min, col, col_max, col_min
    // );
    for i in row_min..row_max {
        for j in col_min..col_max {
            if matrix[i][j] != '.' && !matrix[i][j].is_numeric() {
                return true;
            }
        }
    }
    return false;
}

fn check_surounding_number(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let row_size = matrix.len();
    let col_size = matrix[row].len();
    let col_min = if col == 0 { 0 } else { col - 1 };
    let col_max = if col + 1 == col_size {
        col + 1
    } else {
        col + 2
    };
    let row_min = if row == 0 { 0 } else { row - 1 };
    let row_max = if row + 1 == row_size {
        row + 1
    } else {
        row + 2
    };
    // println!(
    //     "row {}, max {}, min {}\ncol {}, max {}, min {}",
    //     row, row_max, row_min, col, col_max, col_min
    // );
    let mut n1: i32 = 0;
    let n2: i32;

    for i in row_min..row_max {
        let mut j = col_min;
        while j < col_max {
            if matrix[i][j].is_numeric() {
                if n1 == 0 {
                    n1 = find_number(&matrix, i, j);
                    while j < col_max {
                        if matrix[i][j].is_numeric() {
                            j += 1
                        } else {
                            break;
                        }
                    }
                } else {
                    n2 = find_number(&matrix, i, j);
                    return n1 * n2;
                }
            }
            j += 1
        }
    }
    return 0;
}

fn find_number(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut i = col;
    let mut s = String::new();
    while i > 0 {
        if !matrix[row][i - 1].is_numeric() {
            break;
        }
        i -= 1;
    }
    while i < matrix[row].len() {
        if !matrix[row][i].is_numeric() {
            break;
        }
        s.push(matrix[row][i]);
        i += 1;
    }
    return s.parse::<i32>().unwrap();
}
