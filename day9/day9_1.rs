// Answer is 480
use std::fs;

const FILE_NAME: &str = "day9.txt";
const LOW_POINT_RISK_LEVEL_INCREASE: &u8 = &1;
const RADIX: u32  = 10;

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    let heights: Vec::<u8> = file_contents
        .replace('\n', "")
        .chars()
        .map(|char| char.to_digit(RADIX).unwrap() as u8)
        .collect();

    let row_height_count  = file_contents.find("\n").unwrap();
    let total_height_count = heights.len();

    let mut low_point_risk_level_sum: u64 = 0;
    for (i, height) in heights.iter().enumerate() {
        let mut adjacent_heights: Vec::<u8> = Vec::new();

        if height_has_left_side(i, row_height_count) {
            adjacent_heights.push(heights[i - 1]);
        }

        if height_has_right_side(i, row_height_count) {
            adjacent_heights.push(heights[i + 1]);
        }

        if height_has_top_side(i, row_height_count) {
            adjacent_heights.push(heights[i - row_height_count]);
        }

        if height_has_bottom_side(i, row_height_count, total_height_count) {
            adjacent_heights.push(heights[i + row_height_count]);
        }

        let mut height_is_low_point = true;
        for adjacent_height in adjacent_heights {
            if height >= &adjacent_height {
                height_is_low_point = false;
            }
        }

        if height_is_low_point {
            low_point_risk_level_sum += (height + LOW_POINT_RISK_LEVEL_INCREASE) as u64;
        }
    }

    println!("{}", low_point_risk_level_sum);
}

fn height_has_left_side(index: usize, line_length: usize) -> bool {
    return !(index % line_length == 0);
}

fn height_has_right_side(index: usize, line_length: usize) -> bool {
    return !((index + 1) % line_length == 0);
}


fn height_has_top_side(index: usize, line_length: usize) -> bool {
    return index >= line_length
}

fn height_has_bottom_side(index: usize, line_length: usize, height_count: usize ) -> bool {
    return index < height_count - line_length;
}
