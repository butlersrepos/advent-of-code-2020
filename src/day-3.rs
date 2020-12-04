use advent_of_code::read_lines;

const TREE: &str = "#";

fn main() {
    let lines = read_lines("day-3-input.txt");

    let paths = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut results: Vec<usize> = vec![];
    for path in paths {
        let trees_hit = solve_for(&lines, path.0, path.1);
        println!("Reached the end and only hit {} trees", trees_hit);
        results.push(trees_hit);
    }

    println!(
        "Multiplied total: {}",
        results.iter().fold(1, |acc, x| acc * x)
    );
}

fn solve_for(lines: &Vec<String>, move_x: usize, move_y: usize) -> usize {
    let width = lines[0].len();

    // Position tracking
    let mut current_y = 0;
    let mut current_x = 0;
    // Our conclusion value
    let mut trees_hit = 0;

    while current_y < lines.len() {
        current_x += move_x;
        current_y += move_y;
        // Finish line condition
        if current_y >= lines.len() {
            break;
        }
        // Wrap horizontal movement
        if current_x >= width {
            current_x = current_x % width;
        }
        // Check our current square for a tree
        if lines[current_y].get(current_x..current_x + 1) == Some(TREE) {
            trees_hit = trees_hit + 1;
        }
    }

    trees_hit
}
