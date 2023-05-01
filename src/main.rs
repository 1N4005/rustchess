use game::board::{Board, STARTPOS};
use engine::search::search;

mod game;
mod engine;

fn main() {
    let mut board: Board = Board::new(STARTPOS);
}