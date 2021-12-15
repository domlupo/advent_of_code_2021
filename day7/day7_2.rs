// Answer is 3101079875
use std::fs;

const FILE_NAME: &str = "day7.txt";

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let min = file_contents
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .min().unwrap();

    let max = file_contents
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .max().unwrap();

    let mut min_fuel_needed: Option<i64> = None;
    for position in min..=max {
        let mut fuel_total = 0;
       
        for crab_position in file_contents.split(",") {
            let position_diff = (crab_position.parse::<i64>().unwrap() - position as i64).abs();

            for i in 0..=position_diff {
                fuel_total += i
            }
        }

        if min_fuel_needed.is_none() || fuel_total < min_fuel_needed.unwrap() {
            min_fuel_needed = Some(fuel_total);
        }
    }

    println!("{}", min_fuel_needed.unwrap().to_string());
}
