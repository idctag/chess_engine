use chess_engine::board::Board;

fn main() {
    let board = Board::new().unwrap();
    let white = board.pieces[0][0];
    println!("white pawns");
    println!("{:064b}", white);
    let black = board.pieces[1][0];
    println!("black pawns");
    println!("{:064b}", black);
}
