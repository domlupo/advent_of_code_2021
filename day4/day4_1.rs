// Answer is TODO

use std::fs;

const FILE_NAME: &str  = "day4.txt";
const BINGO_BOARD_ROWS: i8 = 5;
const BINGO_BOARD_COLS: i8 = 5;

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    let mut unparsed_called_nums: Option<String> = None;
    let mut unparsed_bingo_boards: Vec<UnparsedBingoBoard> = Vec::new();
    let mut unparsed_bingo_board = UnparsedBingoBoard { ..Default::default() };

    for line in file_contents.split("\n") {
        if unparsed_called_nums.is_none() {
            unparsed_called_nums = Some(line.to_string());
        } else if line != "" {
            unparsed_bingo_board.set_next_line(line.to_string());

            if unparsed_bingo_board.ready_to_copy {
                unparsed_bingo_boards.push(unparsed_bingo_board.clone());
                unparsed_bingo_board.ready_to_copy = false;
            }
        }
    }

    println!("{}", unparsed_called_nums.unwrap());
}

#[derive(Clone)]
struct UnparsedBingoBoard {
    lines: [String; BINGO_BOARD_ROWS as usize],
    ready_to_copy: bool,
    index: usize,
}

impl Default for UnparsedBingoBoard {
    fn default() -> UnparsedBingoBoard {
        UnparsedBingoBoard {
            lines: Default::default(),
            ready_to_copy: false,
            index: 0,
        }
    }
}

impl UnparsedBingoBoard {
    fn set_next_line(&mut self, line: String) {
        self.lines[self.index] = line;

        if self.index + 1  == self.lines.len() {
            self.index = 0;
            self.ready_to_copy = true;
        } else {
            self.index += 1;
        }
    }
}

struct BingoSquare {
    val: i64,
    marked: bool,
}

struct BingoBoard {
    board: [BingoSquare; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize ],
}

struct BingoBoardResult {
    sum_uncalled_nums: i64,
    last_num_called: i64,
    count_called_nums: i64,
}
