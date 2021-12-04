use array2d::Array2D;
use itertools::Itertools;

const BOARDSIZE: usize = 5;
type Board = [[u8; BOARDSIZE]; BOARDSIZE];
type WinningBoard = usize;

struct BoardState {
    filled: Array2D<bool>,
    won: bool,
}

struct Game {
    draws: Vec<u8>,
    boards: Vec<Board>,
}

fn parse(fname: &str) -> Game {
    let input = std::fs::read_to_string(fname).unwrap();
    let mut lines = input.lines();
    let draws: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|draw_str| draw_str.parse().ok())
        .collect();
    let boards: Vec<[[u8; 5]; 5]> = lines
        .filter(|&line_str| !line_str.is_empty())
        .chunks(5)
        .into_iter()
        .map(|chunk_line_iter| {
            chunk_line_iter
                .map(|board_row_str| {
                    board_row_str
                        .split_ascii_whitespace()
                        .filter_map(|entry| entry.parse::<u8>().ok())
                        .collect::<Vec<u8>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u8; 5]>>()
                .try_into()
                .unwrap()
        })
        .collect();
    Game {
        draws: draws,
        boards: boards,
    }
}
fn check_win(state: &mut BoardState) -> bool {
    let row_win = state
        .filled
        .as_rows()
        .iter()
        .map(|row| row.iter().all(|&v| v))
        .any(|rows| rows);
    let col_win = state
        .filled
        .as_columns()
        .iter()
        .map(|col| col.iter().all(|&v| v))
        .any(|cols| cols);
    row_win | col_win
}

fn play_round(draw: u8, boards: &Vec<Board>, states: &mut [BoardState]) -> Option<WinningBoard> {
    let mut last_win: Option<WinningBoard> = None;
    for (board_num, &board) in boards.iter().enumerate() {
        if states[board_num].won {
            continue;
        }
        'board: for (i, &row) in board.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == draw {
                    let _ = states[board_num].filled.set(i, j, true).unwrap();
                    if check_win(&mut states[board_num]) {
                        states[board_num].won = true;
                        last_win = Some(board_num);
                    }
                    break 'board;
                }
            }
        }
    }
    last_win
}

fn sum_of_unmarked(board: &Board, state: &BoardState) -> usize {
    let mut result: usize = 0;
    for (i, &row) in board.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if !state.filled.get(i, j).unwrap() {
                result += col as usize;
            }
        }
    }
    result
}

fn part1(game: Game) -> usize {
    let num_boards = game.boards.len();
    let mut board_states: Vec<BoardState> = (0..num_boards)
        .map(|_| BoardState {
            filled: Array2D::filled_with(false, BOARDSIZE, BOARDSIZE),
            won: false,
        })
        .collect::<Vec<BoardState>>();
    for draw in game.draws {
        let result = play_round(draw, &game.boards, &mut board_states[..]);
        if let Some(winning_board_num) = result {
            return sum_of_unmarked(
                &game.boards[winning_board_num],
                &board_states[winning_board_num],
            ) * (draw as usize);
        }
    }
    panic!("nothing won");
}

fn part2(game: Game) -> usize {
    let num_boards = game.boards.len();
    let mut board_states: Vec<BoardState> = (0..num_boards)
        .map(|_| BoardState {
            filled: Array2D::filled_with(false, BOARDSIZE, BOARDSIZE),
            won: false,
        })
        .collect::<Vec<BoardState>>();
    let mut last_winning_value: usize = 0;

    for draw in game.draws {
        let result = play_round(draw, &game.boards, &mut board_states[..]);
        if let Some(winning_board_num) = result {
            last_winning_value = sum_of_unmarked(
                &game.boards[winning_board_num],
                &board_states[winning_board_num],
            ) * (draw as usize);
        }
    }
    last_winning_value
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 4512)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 1924)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
