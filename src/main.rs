use chess_engine::board::Board;

fn main() {
    let board = Board::new().unwrap();
    println!("{:?}", board)
}
