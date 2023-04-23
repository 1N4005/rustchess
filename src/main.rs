mod chess;

fn main() {
    let board = chess::board::Board::new(chess::board::STARTPOS);
    board.draw();
}
