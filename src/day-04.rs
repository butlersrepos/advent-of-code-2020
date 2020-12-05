use regex::Regex;
use std::fs;

const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    // Separate passport attempts
    let submission_regex: Regex = Regex::new(r"(?m)^\r$").unwrap();
    // Separate passport pieces
    let pieces_regex: Regex = Regex::new(r"\s|\n").unwrap();

    let contents =
        fs::read_to_string("day-04-input.txt").expect("Failed to read the file contents!");

    let raw_submissions: Vec<String> = submission_regex
        .split(contents.as_str())
        .map(|x| x.trim().to_string())
        .collect();
    let total = raw_submissions.len();

    for submission in raw_submissions.iter().by_ref() {
        println!("'{}'", submission);
        println!("------------");
    }

    println!("Total passport attempts: {}", total);

    let submissions = raw_submissions.iter().map(|s| {
        let cleaned = s.replace("\n", "").to_string();
        pieces_regex
            .split(cleaned.trim())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    });

    let mut valid_submissions = 0;

    // Go over every submission
    'submissions: for submission in submissions {
        // Get the list of fields this submission has, (ignore the values I guess)
        let field_keys: Vec<String> = submission
            .iter()
            .map(|x| x.split(":").map(|s| s.to_string()).collect::<Vec<String>>())
            .map(|x| x.get(0).unwrap().clone())
            .collect();

        // Verify that we have all required fields
        for required_field in REQUIRED_FIELDS.iter() {
            if !field_keys.contains(&required_field.to_string()) {
                continue 'submissions;
            }
        }
        // Only count it if we got here, by checking every REQUIRED_FIELDS item
        valid_submissions = valid_submissions + 1;
    }
    
    println!("Total valid submissions: {}", valid_submissions);
}
