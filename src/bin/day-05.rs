use advent_of_code::read_lines;

const LOWER_ROW: char = 'F';
// const HIGHER_ROW: char = 'B';

const LOWER_SEAT: char = 'L';
// const HIGHER_SEAT: char = 'R';

fn convert_to_binary_num(s: &str, low_marker: &char) -> isize {
    let binary_string = s
        .chars()
        .into_iter()
        .map(|x| if &x == low_marker { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("");

    isize::from_str_radix(binary_string.as_str(), 2).unwrap()
}

fn main() {
    let lines = read_lines("day-05-input.txt");
    let mut highest_seat_id = 0;

    for entry in lines {
        let row_code = entry.get(0..7).unwrap();
        let seat_code = entry.get(7..).unwrap();

        let row = convert_to_binary_num(row_code, &LOWER_ROW);
        let seat = convert_to_binary_num(seat_code, &LOWER_SEAT);
        let seat_id = row * 8 + seat;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }

        println!("Row number: {}, seat: {}, seat id: {}", row, seat, seat_id);
    }

    println!("Highest seat ID: {}", highest_seat_id);
}
