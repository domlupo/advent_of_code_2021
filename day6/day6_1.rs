// Answer is 388739
use std::fs;

const FILE_NAME: &str = "day6.txt";
const NEW_FISH_START_DAY: &u8 = &8;
const FISH_RESET_DAY: &u8 = &6;
const DAYS_TO_MULTIPLY: &u8 = &80;

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let mut total_fish_count = 0u64;
    for fish_day in file_contents.split(",") {
        total_fish_count += get_fish_count(fish_day.parse::<u8>().unwrap(), *DAYS_TO_MULTIPLY);
    }

    println!("{}", total_fish_count.to_string());
}

fn get_fish_count(fish_start_day: u8, days_to_multiply: u8) -> u64 {
    let mut fish_days = vec![fish_start_day];

    for _ in 0..days_to_multiply {
        for i in 0..fish_days.len() {
            if fish_days[i] == 0u8 {
                fish_days[i] = *FISH_RESET_DAY;
                fish_days.push(*NEW_FISH_START_DAY);
            } else {
                fish_days[i] = (fish_days[i] - 1).clone();
            }
        }
    }

    return fish_days.len() as u64;
}
