mod lib;
use advent_of_code::read_lines_as_nums;

fn main() {
    let filename: String = String::from("day-01-input.txt");

    let nums: Vec<u32> = read_lines_as_nums(filename);

    println!("Our total: {}", nums.iter().fold(0, |acc, x| acc + x));
    find_2020_in_two(nums.clone());
    find_2020_in_three(nums.clone());
}

fn find_2020_in_two(nums: Vec<u32>) {
    for n in &nums {
        for m in &nums {
            if n + m == 2020 {
                println!("Found combination of {} and {}!", n, m);
                println!("Multiplied total would be {}", n * m);
                return;
            }
        }
    }
}

fn find_2020_in_three(nums: Vec<u32>) {
    for a in &nums {
        for b in &nums {
            for c in &nums {
                if a + b + c == 2020 {
                    println!("Found combination of {}, {}, and {}!", a, b, c);
                    println!("Multiplied total would be {}", a * b * c);
                    return;
                }
            }
        }
    }
}
