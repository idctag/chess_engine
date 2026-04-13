use crate::{
    fen::{FenError, populate_board},
    piece::Color,
};

const INITIAL_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    pub pieces: [[u64; 6]; 2],

    pub turn: Option<Color>,
}

impl Board {
    pub fn new() -> Result<Self, FenError> {
        Board::from_fen(INITIAL_BOARD_FEN)
    }

    pub fn empty_board() -> Self {
        Board {
            pieces: [[0; 6]; 2],
            turn: None,
        }
    }

    pub fn from_fen(raw: &str) -> Result<Self, FenError> {
        let piece_placement = raw
            .split_whitespace()
            .next()
            .ok_or(FenError::InvalidStructure)?;

        let chars: Vec<char> = piece_placement.chars().filter(|&c| c != '/').collect();
        let board = populate_board(&chars)?;
        Ok(board)
    }
}
