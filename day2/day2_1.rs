// Answer is 2272262

use std::fs;

fn main() {
    let contents = fs::read_to_string("day2.txt")
        .expect("Something went wrong reading the file");

    let mut horizontal = 0;
    let mut depth = 0;

    for submarine_vector in contents.split("\n") {
        if submarine_vector == "" {
            break;
        }

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

    println!("{}", horizontal * depth);
}
