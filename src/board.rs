use core::fmt;

use crate::{
    fen::{FenError, piece_char, populate_board},
    piece::{Color, Piece},
};

const INITIAL_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    squares: [Option<(Piece, Color)>; 64],
}

impl Board {
    pub fn new() -> Result<Self, FenError> {
        Board::from_fen(INITIAL_BOARD)
    }

    pub fn from_fen(raw: &str) -> Result<Self, FenError> {
        let piece_placement = raw
            .split_whitespace()
            .next()
            .ok_or(FenError::InvalidStructure)?;

        let chars: Vec<char> = piece_placement.chars().filter(|&c| c != '/').collect();
        let squares = populate_board(&chars)?;
        Ok(Board { squares })
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (i, square) in self.squares.iter().enumerate() {
            if i % 8 == 0 && i > 0 {
                writeln!(f)?;
            }
            match square {
                Some((piece, Color::White)) => write!(f, " {} ", piece_char(*piece))?,
                Some((piece, Color::Black)) => {
                    write!(f, " {} ", piece_char(*piece).to_ascii_lowercase())?
                }
                None => write!(f, " . ")?,
            }
        }
        Ok(())
    }
}
