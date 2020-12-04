use advent_of_code::read_lines;

fn main() {
    const TREE: &str = "#";
    // Velocity
    const MOVE_X: usize = 3;
    const MOVE_Y: usize = 1;

    let lines = read_lines("day-3-input.txt");
    let width = lines[0].len();

    // Position tracking
    let mut current_y = 0;
    let mut current_x = 0;
    // Our conclusion value
    let mut trees_hit = 0;

    while current_y < lines.len() {
        current_x += MOVE_X;
        current_y += MOVE_Y;
        // Finish line condition
        if current_y == lines.len() {
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

    println!("Reached the end and only hit {} trees", trees_hit);
}
