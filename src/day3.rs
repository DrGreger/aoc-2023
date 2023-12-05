use crate::read_input;

pub fn print_solution() {
    static DAY: &str = "day3_ex";
    let input = read_input(DAY);
    let lines = input.split("\n").collect::<Vec<&str>>();

    print!("\n--- {} ---\nAnswere to part one: ", DAY,);
    println!("{}", part_one(lines.clone()));
    print!("\nAnswere to part two: ");
    println!("{}\n---", part_two(lines.clone()))
}

fn part_one(lines: Vec<&str>) -> String {
    let mut matrix: Vec<Vec<char>> = create_matrix(lines);

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
