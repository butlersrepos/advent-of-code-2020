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

    let mut seat_ids = lines
        .into_iter()
        .map(|x| {
            let row_code = x.get(0..7).unwrap();
            let seat_code = x.get(7..).unwrap();

            let row = convert_to_binary_num(row_code, &LOWER_ROW);
            let seat = convert_to_binary_num(seat_code, &LOWER_SEAT);
            let seat_id = row * 8 + seat;

            seat_id
        })
        .collect::<Vec<isize>>();

    seat_ids.sort();

    let mut prev_id = -1;
    for id in seat_ids {
        let is_first_item = prev_id == -1;
        let is_first_row = id <= 15;
        let is_the_next_seat = prev_id + 1 == id;
        println!("Checking seat {}...", id);
        // Get an initial previous and really begin, also ignore first row
        if is_first_item || is_first_row || is_the_next_seat {
            prev_id = id;
            continue;
        } else {
            println!("Your seat id is: {}", id);
            break;
        }
    }
}
