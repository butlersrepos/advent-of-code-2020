use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs};

struct Bag {
    name: String,
    contents: Option<Vec<(Bag, usize)>>,
}

lazy_static! {
    static ref BAG_DEF: Regex = Regex::new(r"^(\w+\s\w+)\sbags contain (.+)$").unwrap();
    static ref INNER_BAG: Regex = Regex::new(r"(\d) (.+) bag[s]{0,1}[.]{0,1}$").unwrap();
}

fn main() {
    let contents = fs::read_to_string("day-07-input.txt").expect("Failed to read file");
    let lines = contents
        .split("\r\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut bag_defs: HashMap<String, Bag> = HashMap::new();

    // Process rules and build up Bag Defs
    for line in lines {
        let captures = BAG_DEF.captures(&line).unwrap();

        let bag_type = captures.get(1).unwrap().as_str();
        let bag_contents = captures.get(2).unwrap().as_str();

        println!("Bags of type {} contain {}", bag_type, bag_contents);

        if bag_contents == "no other bags." {
            bag_defs.insert(
                String::from(bag_type),
                Bag {
                    name: String::from(bag_type),
                    contents: None,
                },
            );
            continue;
        }

        let inner_bags = bag_contents
            .split(", ")
            .map(|x| x.to_string())
            .map(|x| {
                println!("{}", x);
                let inner_captures = INNER_BAG.captures(&x).unwrap();
                let quantity = inner_captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let name = String::from(inner_captures.get(2).unwrap().as_str());
                (
                    Bag {
                        name: name,
                        contents: None,
                    },
                    quantity,
                )
            })
            .collect::<Vec<(Bag, usize)>>();

        bag_defs.insert(
            String::from(bag_type),
            Bag {
                name: String::from(bag_type),
                contents: Some(inner_bags),
            },
        );
    }

    // Check every Bag rule for eventual Shiny Gold.. wait... shouldn't this be a tree walking problem and not brute force?
}
