// Answer is 1118645287
use std::fs;

const FILE_NAME: &str = "day10.txt";

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let mut incomplete_lines: Vec<Vec<char>> = Vec::new();
    for line in file_contents.lines() {
        let mut char_stack: Vec<char> = Vec::new();
        let mut corrupted_line: bool = false;

        for c in line.chars() {
            if is_left_char(c) {
                char_stack.push(c);
            } else if is_right_char(c) {

                if char_stack.len() == 0 {
                    corrupted_line = true;
                    break;
                }

                let left_char = char_stack.pop().unwrap();

                if is_illegal_char_pair(left_char, c) {
                    corrupted_line = true;
                    break;
                }
            } else {
                panic!("An internal error occurred.");
            }
        }

        if corrupted_line == false {
            incomplete_lines.push(char_stack);
        }
    }

    let mut incomplete_line_scores: Vec<usize> = Vec::new();
    for line in incomplete_lines {
        let mut incomplete_line_score = 0;
        for c in line.iter().rev() {
            incomplete_line_score *= 5;
            incomplete_line_score += get_completion_char_val(*c);
        }

        incomplete_line_scores.push(incomplete_line_score);
    }

    let median_index = incomplete_line_scores.len() / 2;
    incomplete_line_scores.sort();
    let median_incomplete_line_score = incomplete_line_scores[median_index];

    println!("{}", median_incomplete_line_score);
}

fn is_left_char(c: char) -> bool {
    if c == '(' || c == '[' || c == '{' || c == '<' {
        return true;
    } else {
        return false;
    }
}


fn is_right_char(c: char) -> bool {
    if c == ')' || c == ']' || c == '}' || c == '>' {
        return true;
    } else {
        return false;
    }
}

fn is_illegal_char_pair(left_char: char, right_char: char) -> bool {
    if left_char == '(' && right_char == ')' {
        return false;
    } else if left_char == '[' && right_char == ']' {
        return false;
    } else if left_char == '{' && right_char == '}' {
        return false;
    } else if left_char == '<' && right_char == '>' {
        return false;
    } else {
        return true;
    }
}

fn get_completion_char_val(c: char) -> usize {
    if c == '(' {
        return 1;
    } else if c == '[' {
        return 2;
    } else if c == '{' {
        return 3;
    } else if c == '<' {
        return 4;
    }

    panic!("This char does not have a value since it is legal.");
}
