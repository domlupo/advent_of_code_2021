// Answer is 7075
use std::fs;

const FILE_NAME: &str  = "day4.txt";
const BINGO_BOARD_ROWS: u8 = 5;
const BINGO_BOARD_COLS: u8 = 5;

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

    let called_nums: Vec<u8> = unparsed_called_nums
        .unwrap()
        .split(",")
        .map(|str| str.parse::<u8>().unwrap())
        .collect();

    let bingo_board_results: Vec<BingoBoardResult> = bingo_boards
        .into_iter()
        .map(|mut bingo_board| bingo_board.get_result(&called_nums))
        .collect();

    let mut most_called_nums_with_bingo: Option<u64> = None;
    let mut winning_board_score: Option<u64> = None;

    for bingo_board_result in bingo_board_results {
        if bingo_board_result.bingo {
            if most_called_nums_with_bingo.is_none() {
                most_called_nums_with_bingo = Some(bingo_board_result.count_called_nums);
                winning_board_score = Some(bingo_board_result.sum_uncalled_nums *
                                           bingo_board_result.last_num_called);
            } else if bingo_board_result.count_called_nums > most_called_nums_with_bingo.unwrap() {
                most_called_nums_with_bingo = Some(bingo_board_result.count_called_nums);
                winning_board_score = Some(bingo_board_result.sum_uncalled_nums *
                                           bingo_board_result.last_num_called);
            }
        }
    }

    println!("{}", winning_board_score.unwrap().to_string());
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
                    parsed_bingo_board[i] = num.parse::<u8>().unwrap();
                    i += 1;
                }
            }
        }

        return parsed_bingo_board;
    }
}

struct BingoSquare {
    num: Option<u8>,
    called: bool,
}
impl Default for BingoSquare { fn default() -> BingoSquare { BingoSquare { num: None,
            called: false,
        }
    }
}

struct BingoBoard {
    squares: [BingoSquare; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize],
}

impl BingoBoard {
    fn build (bingo_nums: [u8; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize]) -> BingoBoard {
        let mut squares:
            [BingoSquare; (BINGO_BOARD_ROWS * BINGO_BOARD_COLS) as usize] = Default::default();
        let mut i = 0;

        for num in bingo_nums {
            squares[i].num = Some(num);
            i += 1;
        }

        return BingoBoard { squares: squares }
    }

    fn get_result (&mut self, called_nums: &Vec<u8>) -> BingoBoardResult {
        let mut last_num_called: Option<u64> = None;
        let mut count_called_nums: u64 = 0;
        let mut bingo = false;

        for (i, num) in called_nums.iter().enumerate() {
            last_num_called = Some(u64::from(num.clone()));
            count_called_nums = i as u64;

            self.mark_called_num(num);
            if self.bingo() {
                bingo = true;
                break;
            }
        }

        let mut sum_uncalled_nums = 0u64;

        for bingo_square in &mut self.squares {
            if bingo_square.called == false {
                sum_uncalled_nums += bingo_square.num.unwrap() as u64;
            }
        }

        return BingoBoardResult {
            sum_uncalled_nums: sum_uncalled_nums,
            last_num_called: last_num_called.unwrap(),
            count_called_nums: count_called_nums,
            bingo: bingo,
        }
    }

    fn mark_called_num (&mut self, called_num: &u8) -> () {
        for mut bingo_square in &mut self.squares {
            if bingo_square.num.unwrap() == *called_num {
                bingo_square.called = true;
                return;
            }
        }
    }

    fn bingo (&self) -> bool {
        if self.row_bingo() || self.col_bingo() {
            return true;
        }

        return false;
    }

    fn row_bingo (&self) -> bool {
        for i in 0..BINGO_BOARD_ROWS {
            let mut called_count = 0;

            for j in 0..BINGO_BOARD_COLS {
                if self.squares[((i * BINGO_BOARD_COLS) + j) as usize].called {
                    called_count += 1;
                } else {
                    break;
                }
            }

            if called_count == BINGO_BOARD_COLS {
                return true;
            }
        }

        return false;
    }

    fn col_bingo (&self) -> bool {
        for i in 0..BINGO_BOARD_COLS {
            let mut called_count = 0;

            for j in 0..BINGO_BOARD_ROWS {
                if self.squares[((j * BINGO_BOARD_COLS) + i) as usize].called {
                    called_count += 1;
                } else {
                    break;
                }
            }

            if called_count == BINGO_BOARD_ROWS {
                return true;
            }
        }

        return false;
    }
}

struct BingoBoardResult {
    sum_uncalled_nums: u64,
    last_num_called: u64,
    count_called_nums: u64,
    bingo: bool,
}
