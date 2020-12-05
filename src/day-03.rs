use advent_of_code::read_lines;
use std::env;
const TREE: &str = "#";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_args = &args[1..];
    let lines = read_lines("day-03-input.txt");

    let paths: Vec<(usize, usize)> = path_args.iter().map(convert_velocities).collect();

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

fn convert_velocities(arg: &String) -> (usize, usize) {
    let new_coord = arg
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (*new_coord.get(0).unwrap(), *new_coord.get(1).unwrap())
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
