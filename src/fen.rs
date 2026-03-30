use core::fmt;

use crate::piece::{Color, Piece};

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

pub fn populate_board(chars: &[char]) -> Result<[Option<(Piece, Color)>; 64], FenError> {
    let mut board = [None; 64];
    let mut index = 0;
    for c in chars {
        match c {
            '1'..='8' => index += c.to_digit(10).unwrap() as usize,
            'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => {
                board[index] = Some((parse_piece(*c)?, Color::White));
                index += 1
            }
            'p' | 'r' | 'n' | 'b' | 'q' | 'k' => {
                board[index] = Some((parse_piece(*c)?, Color::Black));
                index += 1
            }
            _ => return Err(FenError::InvalidCharacter),
        }
    }

    Ok(board)
}

pub fn parse_piece(piece_char: char) -> Result<Piece, FenError> {
    match piece_char.to_ascii_lowercase() {
        'p' => Ok(Piece::Pawn),
        'r' => Ok(Piece::Rook),
        'n' => Ok(Piece::Knight),
        'b' => Ok(Piece::Bishop),
        'q' => Ok(Piece::Queen),
        'k' => Ok(Piece::King),
        _ => Err(FenError::InvalidCharacter),
    }
}

pub fn piece_char(piece: Piece) -> char {
    match piece {
        Piece::King => 'K',
        Piece::Queen => 'Q',
        Piece::Rook => 'R',
        Piece::Bishop => 'B',
        Piece::Pawn => 'P',
        Piece::Knight => 'N',
    }
}
