use core::fmt;

#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

enum FenError {
    InvalidStructure,
    InvalidCharacter,
}

struct Board {
    squares: [Option<(Piece, Color)>; 64],
}

const INITIAL_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

impl fmt::Debug for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::InvalidStructure => write!(f, "Invalid FEN structure"),
            FenError::InvalidCharacter => write!(f, "Invalid character in FEN"),
        }
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
fn piece_char(piece: Piece) -> char {
    match piece {
        Piece::King => 'K',
        Piece::Queen => 'Q',
        Piece::Rook => 'R',
        Piece::Bishop => 'B',
        Piece::Pawn => 'P',
        Piece::Knight => 'N',
    }
}

fn populate_board(chars: &[char]) -> Result<[Option<(Piece, Color)>; 64], FenError> {
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

fn parse_piece(piece_char: char) -> Result<Piece, FenError> {
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

fn main() {
    let board = Board::new().unwrap();
    println!("{:?}", board)
}
