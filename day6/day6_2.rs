// Answer is 1741362314973
use std::fs;
use std::collections::HashMap;

const FILE_NAME: &str = "day6.txt";
const NEW_FISH_START_DAY: &usize = &8;
const FISH_RESET_DAY: &usize = &6;
const DAYS_TO_MULTIPLY: &usize = &256;

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let mut fish_day_counts: HashMap<usize, u64> = HashMap::new();

    for fish_day_str in file_contents.split(",") {
        let fish_day = fish_day_str.parse::<usize>().unwrap();

        if fish_day_counts.contains_key(&fish_day) {
            fish_day_counts.insert(fish_day, fish_day_counts.get(&fish_day).unwrap() + 1 as u64);
        } else {
            fish_day_counts.insert(fish_day, 1);
        }
    }

    for i in 0..=*NEW_FISH_START_DAY {
        if !fish_day_counts.contains_key(&i) {
            fish_day_counts.insert(i, 0);
        }
    }

    for _ in 0..*DAYS_TO_MULTIPLY {
        let num_new_latern_fish = fish_day_counts.get(&(0 as usize)).unwrap().clone();

        for i in 1..=*NEW_FISH_START_DAY {
            if i == FISH_RESET_DAY + 1 {
                fish_day_counts.insert(i - 1, *fish_day_counts.get(&i).unwrap() + num_new_latern_fish);
            } else {
                fish_day_counts.insert(i - 1, *fish_day_counts.get(&i).unwrap());
            }
        }

        fish_day_counts.insert(*NEW_FISH_START_DAY, num_new_latern_fish);
    }

    let mut total_fish_count = 0;
    for day_count in fish_day_counts.values() {
        total_fish_count += day_count;
    }

    println!("{}", total_fish_count);
}
