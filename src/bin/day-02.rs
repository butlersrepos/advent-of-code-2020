use advent_of_code::read_lines;
use regex::Regex;

fn main() {
    let line_regex: Regex = Regex::new(r"(\d+)-(\d+) (\w): (.+)").unwrap();
    let lines = read_lines("day-02-input.txt");
    let results: Vec<bool> = lines
        .iter()
        .map(|x| {
            if !line_regex.is_match(x) {
                println!("Line {} is malformed!", x);
                return false;
            } else {
                let captures = line_regex.captures(x).unwrap();
                let position1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let position2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let letter = captures.get(3).unwrap().as_str();
                let password = String::from(captures.get(4).unwrap().as_str().trim());

                let (_, char1) = password.char_indices().nth(position1 - 1).unwrap();
                let (_, char2) = password.char_indices().nth(position2 - 1).unwrap();
                if char1.to_string() == letter && char2.to_string() == letter {
                    return false;
                }
                if char1.to_string() != letter && char2.to_string() != letter {
                    return false;
                }
                return true;
            }
        })
        .collect();

    let valids = results
        .iter()
        .fold(0, |acc, x| if *x { acc + 1 } else { acc });
    println!("{} valid passwords", valids);
    println!("{} invalid passwords", results.len() - valids);
}
