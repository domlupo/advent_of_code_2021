// Answer is 1781

use std::fs;

const FILE_NAME: &str  = "day1.txt";

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");
   
    let mut prior_num1: Option<i32> = None;
    let mut prior_num2: Option<i32> = None;
    let mut prior_num3: Option<i32> = None;

    let mut window_increased = 0;

    for line in file_contents.lines() {
        if line == "" {
            break;
        }

        if prior_num3.is_some() {
            let current_num = line.parse::<i32>().unwrap();
            let current_window = current_num + prior_num1.unwrap() + prior_num2.unwrap();
            let prior_window = prior_num1.unwrap() + prior_num2.unwrap() + prior_num3.unwrap();
            
            if current_window > prior_window {
                window_increased += 1;
            }

        }

        if prior_num1.is_none() {
            prior_num1 = Some(line.parse::<i32>().unwrap());
        } else if prior_num2.is_none() {
            prior_num2 = prior_num1;
            prior_num1 = Some(line.parse::<i32>().unwrap());
        } else {
            prior_num3 = prior_num2;
            prior_num2 = prior_num1;
            prior_num1 = Some(line.parse::<i32>().unwrap());
        }
    }

    println!("{}", window_increased);
}
