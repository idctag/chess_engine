enum FenError {
    InvalidPiece,
    InvalidStructure,
}

struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,

    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,

    side_to_move: Color,
}

enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

enum Color {
    White,
    Black,
}

const INITIAL_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";

struct Square {
    piece: Option<(Piece, Color)>,
}

impl Board {
    pub fn from_fen(raw: &str) {
        let ranks: Vec<&str> = raw.split("/").collect();
        if ranks.len() != 6 {
            println!("Not a valid FEN")
        }
    }
}

fn parse_piece_placement(field: &str) -> 

fn main() {
    let board = Board::from_fen(INITIAL_BOARD);
}
