use core::fmt;

use crate::board::Board;

pub enum FenError {
    InvalidStructure,
    InvalidCharacter,
}

impl fmt::Debug for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::InvalidStructure => write!(f, "Invalid FEN structure"),
            FenError::InvalidCharacter => write!(f, "Invalid character in FEN"),
        }
    }
}

pub fn populate_board(chars: &[char]) -> Result<Board, FenError> {
    let mut board = Board::empty_board();
    let mut square = 64;
    for c in chars {
        square -= 1;
        let color = if c.is_ascii_uppercase() { 0 } else { 1 };
        let piece_type = match c.to_ascii_lowercase() {
            'p' => 0,
            'n' => 1,
            'b' => 2,
            'r' => 3,
            'q' => 4,
            'k' => 5,
            '1'..'8' => {
                square -= (c.to_digit(10).unwrap() - 1) as i32;
                continue;
            }
            _ => continue,
        };
        board.pieces[color][piece_type] |= 1 << square
    }
    Ok(board)
}
