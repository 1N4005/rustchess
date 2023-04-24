use game::{board::{parse_square, Board, Move, PieceTypes}, movegen};
use std::io;

mod game;

fn main() {
    let mut board = Board::new(game::board::STARTPOS);
    let mut buffer = String::new();

    loop {
        board.draw();
        let legal_moves: Vec<Move> = movegen::generate_legal_moves(&board);
        println!("{:?}", legal_moves.len());
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                print!("{} bytes read: {}", n, buffer);
            }
            Err(_) => {
                panic!("whoops");
            }
        }
        let m = buffer.trim().split_at(2);
        let mut promotion: PieceTypes = PieceTypes::Empty;
        let prom = m.1.split_at(2).1; 
        match prom {
            "n" => {promotion = PieceTypes::Knight},
            "b" => {promotion = PieceTypes::Bishop},
            "r" => {promotion = PieceTypes::Rook},
            "q" => {promotion = PieceTypes::Queen},
            _ => {}
        }
        let move_to_make = Move {
            from: parse_square(m.0),
            to: parse_square(m.1.split_at(2).0),
            promotion: promotion,
        };
        println!("{}", legal_moves.contains(&move_to_make));
        board.push(&move_to_make);

        buffer.clear();
    }
}
