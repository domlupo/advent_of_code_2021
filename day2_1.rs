// INTERESTING

// TODO
// remove s != "" check and use more rust orientated error handling
// better style and more robust

// ANSWER
// 2272262

use std::fs;

fn main() {
    let contents = fs::read_to_string("day2.txt")
        .expect("Something went wrong reading the file");
    let contents_split = contents.split("\n");

    let mut horizontal = 0;
    let mut depth = 0;

    for submarine_vector in contents_split {

        if submarine_vector != "" {
            let direction = submarine_vector
                .split_whitespace()
                .next()
                .unwrap_or("");

            let magnitude = submarine_vector
                .split_whitespace()
                .next_back()
                .unwrap_or("")
                .parse::<i32>()
                .unwrap();

            if direction == "forward" {
                horizontal += magnitude;
            } else if direction == "down" {
                depth += magnitude;
            } else if direction == "up" {
                depth -= magnitude;
            }
        }
    }

    println!("{}", horizontal * depth);
}
