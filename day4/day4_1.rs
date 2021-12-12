// Answer is TODO

use std::fs;

const FILE_NAME: &str  = "day4.txt";

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    let mut unparsed_called_nums: Option<String> = None;
    let mut unparsed_bingo_boards: Vec<UnparsedBingoBoard>;
    let mut unparsed_bingo_board;

    for line in file_contents.split("\n") {
        if unparsed_called_nums.is_none() {
            unparsed_called_nums = Some(line.to_string());
        } else if line == "" {
            unparsed_bingo_board = UnparsedBingoBoard { ..Default::default() };
        } else {
            unparsed_bingo_board = UnparsedBingoBoard { ..Default::default() };
        }
    }

    println!("{}", unparsed_called_nums.unwrap());
}

struct UnparsedBingoBoard {
    lines: [String; 5],
    index: i8,
}

impl Default for UnparsedBingoBoard {
    fn default() -> UnparsedBingoBoard {
        UnparsedBingoBoard {
            lines: Default::default(),
            index: 0,
        }
    }
}

struct BingoSquare {
    val: i64,
    marked: bool,
}

struct BingoBoard {
    board: [BingoSquare; 25],
}

struct BingoBoardResult {
    sum_uncalled_nums: i64,
    last_num_called: i64,
    count_called_nums: i64,
}
