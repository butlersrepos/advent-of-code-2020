use advent_of_code::read_lines;

fn main() {
    let lines = read_lines("day-06-input.txt");

    let groups = lines.split(|x| x == "").collect::<Vec<_>>();

    let values = groups
        .iter()
        .map(|group| {
            let mut single_list = group.join("").chars().collect::<Vec<char>>();
            single_list.sort();
            single_list.dedup();
            println!(
                "List processed from {} to {}",
                group.join(","),
                single_list.clone().into_iter().collect::<String>()
            );
            single_list.len()
        })
        .collect::<Vec<usize>>();

    println!(
        "Total of cumulative totals: {}",
        values.iter().fold(0, |acc, x| acc + x)
    );
}
