// Answer is 1752

use std::fs;

fn main() {
    let file_contents = fs::read_to_string("day1.txt")
        .expect("Something went wrong reading the file");
   
    let mut prior_num: Option<i32> = None;
    let mut num_increased  = 0;

    for line in file_contents.split("\n") {
        if line == "" {
            break;
        }

        if prior_num.is_some() {
            let current_num = line.parse::<i32>().unwrap();

            if current_num > prior_num.unwrap() {
                num_increased += 1;
            }

        }

        prior_num = Some(line.parse::<i32>().unwrap());
    }

    println!("{}", num_increased);
}
