use advent_of_code::read_lines;
use regex::Regex;

fn main() {
    let line_regex: Regex = Regex::new(r"(\d+)-(\d+) (\w): (.+)").unwrap();
    let lines = read_lines("day-2-input.txt");
    let results: Vec<bool> = lines
        .iter()
        .map(|x| {
            if !line_regex.is_match(x) {
                println!("Line {} is malformed!", x);
                return false;
            } else {
                let captures = line_regex.captures(x).unwrap();
                let min = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let max = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let letter = captures.get(3).unwrap().as_str();
                let pass = captures.get(4).unwrap().as_str().trim();

                let occurences = pass.matches(letter).collect::<Vec<&str>>().len() as u32;
                if occurences < min || occurences > max {
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
