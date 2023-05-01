use game::board::{Board, STARTPOS, Move, PieceTypes};
use engine::search::{search, absearch, EvalResult};
use std::io;
use std::time::Instant;

use crate::engine::search;

mod game;
mod engine;

fn main() -> Result<(), ()>{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match &*input.trim() {
        "cli" => {return cli()},
        _ => {
            println!("\"{}\" is not supported.", &*input.trim());
            return Err(());
        },
    }
    Ok(())
}

fn cli() -> Result<(), ()> {
    let mut board = Board::new(STARTPOS);

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        if &*input.trim() == "exit" {
            break;
        }

        board.push(&Move::parse_from(&*input.trim()));

        println!("{}", board);

        let mut eval_result = EvalResult{best_move: Move{from: 0, to: 0, promotion: PieceTypes::Empty}};

        let start = Instant::now();
        // search(4, &mut board, &mut eval_result, 0);
        absearch(6, &mut board, search::REALLY_SMALL_NUMBER, search::REALLY_BIG_NUMBER, &mut eval_result, 0);
        let elapsed = start.elapsed();
        board.push(&eval_result.best_move);

        println!("{}finished in {:.2?}", board, elapsed);
        println!("{}", eval_result.best_move.uci());
    }
    Ok(())
}