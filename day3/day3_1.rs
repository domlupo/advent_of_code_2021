// Answer is 3309596

use std::fs;

const FILE_NAME: &str  = "day3.txt";
const RADIX: u32 = 10;

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let mut zero_counts: [i32; 12] = [0; 12];
    let mut one_counts: [i32; 12] = [0; 12];

    for num_chars in file_contents.lines() {
        let mut index = 0;

        for num_char in num_chars.chars() {
            let num = num_char.to_digit(RADIX).unwrap();
                
            if num == 0 {
                zero_counts[index] += 1;
            } else if num == 1 {
                one_counts[index] += 1;
            }

            index += 1;
        }
    }

    let mut binary_gamma: [i32; 12] = [0; 12];
    let mut binary_epsilon: [i32; 12] = [1; 12];

    for i in 0..12 {
       if one_counts[i] > zero_counts[i] {
           binary_gamma[i] = 1;
           binary_epsilon[i] = 0;
       }
    }

    let base: i32 = 2;
    let mut gamma = 0;
    let mut epsilon = 0;

    for j in 0..12 {
        gamma += base.pow(11 - j) * binary_gamma[j as usize];
        epsilon += base.pow(11 - j) * binary_epsilon[j as usize];
    }

    println!("{}", gamma * epsilon);

}
