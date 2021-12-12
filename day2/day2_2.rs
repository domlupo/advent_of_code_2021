// Answer is 2134882034

use std::fs;

fn main() {
    let file_contents = fs::read_to_string("day2.txt")
        .expect("Something went wrong reading the file");

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for submarine_vector in file_contents.split("\n") {
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
             depth += aim * magnitude;
         } else if direction == "down" {
             aim += magnitude;
         } else if direction == "up" {
             aim -= magnitude;
         }
    }

    println!("{}", horizontal * depth);
}
