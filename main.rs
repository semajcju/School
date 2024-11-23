use std::{io};
use colored::*; 

const ROWS: usize = 6;
const COLUMNS: usize = 7;

fn create_board() -> [[u8; COLUMNS]; ROWS] {
    [[0; COLUMNS]; ROWS]
}

fn drop_piece(board: &mut [[u8; COLUMNS]; ROWS], row: usize, col: usize, piece: u8) {
    board[row][col] = piece;
}

fn is_valid_location(board: &[[u8; COLUMNS]; ROWS], col: usize) -> bool {
    board[0][col] == 0
}

fn get_next_open_row(board: &[[u8; COLUMNS]; ROWS], col: usize) -> usize {
    for r in (0..ROWS).rev() {
        if board[r][col] == 0 {
            return r;
        }
    }
    0
}

fn print_board(board: &[[u8; COLUMNS]; ROWS]) {
    print!("\n");
    for row in board.iter() {
        for &cell in row.iter() {
            if cell == 1{
                print!("{} ", cell.to_string().red());
            }
            else if  cell == 2{
                print!("{} ", cell.to_string().blue());
            }
            else{
                print!("{} ", cell);
            }
        }
        println!();
    }
    println!();
}

fn winning_move(board: &[[u8; COLUMNS]; ROWS], piece: u8) -> bool {
    // Check horizontal locations
    for c in 0..COLUMNS-3 {
        for r in 0..ROWS {
            if board[r][c] == piece && board[r][c+1] == piece && board[r][c+2] == piece && board[r][c+3] == piece {
                return true;
            }
        }
    }
    // Check vertical locations
    for c in 0..COLUMNS {
        for r in 0..ROWS-3 {
            if board[r][c] == piece && board[r+1][c] == piece && board[r+2][c] == piece && board[r+3][c] == piece {
                return true;
            }
        }
    }
    // Check positively sloped diagonals
    for c in 0..COLUMNS-3 {
        for r in 0..ROWS-3 {
            if board[r][c] == piece && board[r+1][c+1] == piece && board[r+2][c+2] == piece && board[r+3][c+3] == piece {
                return true;
            }
        }
    }
    // Check negatively sloped diagonals
    for c in 0..COLUMNS-3 {
        for r in 3..ROWS {
            if board[r][c] == piece && board[r-1][c+1] == piece && board[r-2][c+2] == piece && board[r-3][c+3] == piece {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut board = create_board();
    let mut game_over = false;
    let mut turn = 0;

    print_board(&board);

    while !game_over {
        // Player input
        let col = if turn == 0 {
            println!("Player 1 Make your Selection (1-7):");
            get_input()-1
        } else {
            println!("Player 2 Make your Selection (1-7):");
            get_input()-1
        };

        if is_valid_location(&board, col) {
            let row = get_next_open_row(&board, col);
            drop_piece(&mut board, row, col, if turn == 0 { 1 } else { 2 });

            if winning_move(&board, if turn == 0 { 1 } else { 2 }) {
                println!("PLAYER {} WINS!!", if turn == 0 { 1 } else { 2 });
                game_over = true;
            }

            print_board(&board);

            turn += 1;
            turn %= 2;
        } else {
            println!("Column is full, choose another one.");
        }
    }
}

fn get_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}
