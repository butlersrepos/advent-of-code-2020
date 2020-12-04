use std::fs;

pub fn read_lines<S: Into<String>>(filename: S) -> Vec<String> {
    return fs::read_to_string(filename.into())
        .expect("Something went wrong reading the file!")
        .split("\n")
        .map(|x| String::from(x.trim()))
        .collect();
}

pub fn read_lines_as_nums<S: Into<String>>(filename: S) -> Vec<u32> {
    let lines = read_lines(filename.into());
    lines
        .iter()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect()
}
