use chess_engine::board::Board;

fn main() {
    let board = Board::new().unwrap();
    let white = board.pieces[1][5];
    println!("white king");
    println!("{:064b}", white);
    let black = board.pieces[0][5];
    println!("black king");
    println!("{:064b}", black);
}
