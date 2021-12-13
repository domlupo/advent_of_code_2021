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

    for line in file_contents.lines() {
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

    let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    for unparsed_bingo_board in unparsed_bingo_boards {
        bingo_boards.push(BingoBoard::build(unparsed_bingo_board.get_parsed()));
    }
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

    fn get_parsed(self) -> [u8; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize] {
        let mut parsed_bingo_board:
            [u8; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize] = Default::default();
        let mut i = 0;

        for line in self.lines {
            for num in line.split(" ") {

                // .split does not support multiple delimiters, so handle " 2" -> "", "2"
                if num != "" {
                    parsed_bingo_board[i] = num.as_bytes()[0];
                    i += 1;
                }
            }
        }

        return parsed_bingo_board;
    }
}

struct BingoSquare {
    val: Option<u8>,
    marked: bool,
}

impl Default for BingoSquare {
    fn default() -> BingoSquare {
        BingoSquare {
            val: None,
            marked: false,
        }
    }
}

struct BingoBoard {
    squares: [BingoSquare; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize],
}

impl BingoBoard {
    fn build (bingo_vals: [u8; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize]) -> BingoBoard {
        let mut squares:
            [BingoSquare; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize] = Default::default();
        let mut i = 0;

        for val in bingo_vals {
            squares[i].val = Some(val);
            i += 1;
        }

        return BingoBoard { squares: squares }

    }
}

struct BingoBoardResult {
    sum_uncalled_nums: i64,
    last_num_called: i64,
    count_called_nums: i64,
}
