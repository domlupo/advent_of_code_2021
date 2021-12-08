// INTERESTING
// https://github.com/rust-lang/rust/issues/62358
// I tried to use Option<i32>.contains(&<i32>)
// but realized this was an inprogress issue since 2019

// TODO
// remove s != "" check and use more rust orientated error handling
// better style and more robust

// ANSWER
// 1781

use std::fs;

fn main() {
    let contents = fs::read_to_string("day1.txt")
        .expect("Something went wrong reading the file");
    let contents_split = contents.split("\n");
   
    let mut prior_num1: Option<i32> = None;
    let mut prior_num2: Option<i32> = None;
    let mut prior_num3: Option<i32> = None;
    let mut result = 0;

    for s in contents_split {
        if prior_num3.is_some() && s != "" {
            let current_num = s.parse::<i32>().unwrap();
            let current_window = current_num + prior_num1.unwrap() + prior_num2.unwrap();
            let prior_window = prior_num1.unwrap() + prior_num2.unwrap() + prior_num3.unwrap();
            
            if current_window > prior_window {
                result += 1;
            }

        }

        if s != "" {
            if prior_num1.is_none() {
                prior_num1 = Some(s.parse::<i32>().unwrap());
            } else if prior_num2.is_none() {
                prior_num2 = prior_num1;
                prior_num1 = Some(s.parse::<i32>().unwrap());
            } else {
                prior_num3 = prior_num2;
                prior_num2 = prior_num1;
                prior_num1 = Some(s.parse::<i32>().unwrap());
            }
        }
    }

    println!("{}", result);
}
