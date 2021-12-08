// INTERESTING
// https://github.com/rust-lang/rust/issues/62358
// I tried to use Option<i32>.contains(&<i32>)
// but realized this was an inprogress issue since 2019

// TODO
// remove s != "" check and use more rust orientated error handling
// better style and more robust

// ANSWER
// 1752

use std::fs;

fn main() {
    let contents = fs::read_to_string("day1.txt")
        .expect("Something went wrong reading the file");
    let contents_split = contents.split("\n");
   
    let mut prior_num: Option<i32> = None;
    let mut result = 0;

    for s in contents_split {
        if prior_num.is_some() && s != "" {
            let current_num = s.parse::<i32>().unwrap();

            if current_num > prior_num.unwrap() {
                result += 1;
            }

        }

        if s != "" {
            prior_num = Some(s.parse::<i32>().unwrap());
        }
    }

    println!("{}", result);
}
