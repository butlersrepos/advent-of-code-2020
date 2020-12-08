use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    type ValidatorFn = Box<dyn Fn(&str) -> bool>;
    // Validation functions for each field we care about
    // Also: an entry's presence denotes "REQUIRED"
    let mut validations_map: HashMap<&str, ValidatorFn> = HashMap::new();
    validations_map.insert(
        "byr",
        Box::new(|byr: &str| {
            let value = byr.parse::<i32>().unwrap();
            value >= 1920 && value <= 2002
        }),
    );
    validations_map.insert(
        "iyr",
        Box::new(|iyr: &str| {
            let value = iyr.parse::<i32>().unwrap();
            value >= 2010 && value <= 2020
        }),
    );
    validations_map.insert(
        "eyr",
        Box::new(|eyr: &str| {
            let value = eyr.parse::<i32>().unwrap();
            value >= 2020 && value <= 2030
        }),
    );
    validations_map.insert(
        "hgt",
        Box::new(|hgt: &str| {
            let reg = regex::Regex::new(r"(\d+)(cm|in)").unwrap();
            let captures = match reg.captures(hgt) {
                Some(c) => c,
                None => return false,
            };
            let value = match captures.get(1) {
                Some(v) => v.as_str(),
                None => return false,
            };
            let units = match captures.get(2) {
                Some(u) => u.as_str(),
                None => return false,
            };
            match (units, value.parse::<i32>()) {
                ("in", Ok(x)) => x >= 59 && x <= 76,
                ("cm", Ok(x)) => x >= 150 && x <= 193,
                (_, _) => false,
            }
        }),
    );
    validations_map.insert("hcl", Box::new(|hcl: &str| true));
    validations_map.insert("ecl", Box::new(|ecl: &str| true));
    validations_map.insert("pid", Box::new(|pid: &str| true));

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
        for key in validations_map.keys() {
            if let Some(field) = submission.iter().find(|x| x.starts_with(key)) {
                let value = field.split(":").nth(1).unwrap();
                let validator = validations_map.get(key).unwrap();
                if !validator(value) {
                    println!(
                        "❌ Submission had invalid {}\t\t{}",
                        key,
                        submission.join(" ")
                    );
                    continue 'submissions;
                }
            } else {
                println!(
                    "❌ Submission lacked field {}\t\t{}",
                    key,
                    submission.join(" ")
                );
                // Ensures that we have all required fields
                continue 'submissions;
            }
        }
        println!("✅ Submission looks good!\t\t{}", submission.join(" "));
        // Only count it if we got here, by checking every item in validations_map
        valid_submissions = valid_submissions + 1;
    }

    println!("Total valid submissions: {}", valid_submissions);
}
