use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("day-06-input.txt").unwrap();

    let groups = contents
        .split("\r\n\r\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let values = groups
        .iter()
        .map(|group| {
            let members = group
                .split("\r\n")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let remaining = members
                .iter()
                .fold(members.get(0).unwrap().clone(), |acc, curr| {
                    let set1 = acc.chars().collect::<HashSet<_>>();
                    let set2 = curr.chars().collect::<HashSet<_>>();
                    let survivors = set1.intersection(&set2).collect::<HashSet<_>>();
                    // return them in String form for the next comparison
                    survivors.iter().map(|x| x.to_string()).collect::<String>()
                });
            remaining.len()
        })
        .collect::<Vec<usize>>();

    println!(
        "Total of cumulative totals: {}",
        values.iter().fold(0, |acc, x| acc + x)
    );
}
