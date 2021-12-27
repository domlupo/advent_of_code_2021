// Answer is 366027
use std::fs;

const FILE_NAME: &str = "day10.txt";

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let mut total_error_score = 0;
    for line in file_contents.lines() {
        let mut char_stack: Vec<char> = Vec::new();

        for c in line.chars() {
            if is_left_char(c) {
                char_stack.push(c);
            } else if is_right_char(c) {

                if char_stack.len() == 0 {
                    total_error_score += get_illegal_char_val(c);
                    break;
                }

                let left_char = char_stack.pop().unwrap();

                if is_illegal_char_pair(left_char, c) {
                    total_error_score += get_illegal_char_val(c);
                    break;
                }
            } else {
                panic!("An internal error occurred.");
            }
        }
    }

    println!("{}", total_error_score);
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

fn get_illegal_char_val(c: char) -> usize {
    if c == ')' {
        return 3;
    } else if c == ']' {
        return 57;
    } else if c == '}' {
        return 1197;
    } else if c == '>' {
        return 25137;
    }

    panic!("This char does not have a value since it is legal.");
}
