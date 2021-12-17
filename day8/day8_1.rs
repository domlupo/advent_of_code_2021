// Answer is 488
use std::fs;

const FILE_NAME: &str = "day8.txt";

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let file_split: Vec<&str> = file_contents.split(&['\n', '|'][..]).collect();

    let mut count = 0;
    for i in (0..file_split.len()).step_by(2) {
        for output_val in file_split[i + 1].split(" ") {
            if output_val.len() == 2 || output_val.len() == 3 || output_val.len() == 4 || output_val.len() == 7 {
                count += 1;
            }
        }
    }

    println!("{}", count);

}
