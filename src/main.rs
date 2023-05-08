use game::board::{Board, STARTPOS, Move, PieceTypes};
use engine::search::{absearch, EvalResult};
use std::io;
use std::time::Instant;

use crate::engine::hash::generate_hash;
use crate::engine::search::{self, REALLY_BIG_NUMBER, REALLY_SMALL_NUMBER};
use crate::game::movegen::generate_legal_moves;
use crate::game::perft;
use crate::uci::uci;

mod game;
mod engine;
mod uci;

fn main() -> Result<(), ()>{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match &*input.trim() {
        "cli" => {return cli()},
        "uci" => {return uci()},
        _ => {
            println!("\"{}\" is not supported.", &*input.trim());
            return Err(());
        },
    }
}

fn cli() -> Result<(), ()> {
    let mut board = Board::new("8/4k3/8/3K4/4P3/8/8/8 w - - 0 1");
    // println!("{:?}", board.zobrist_table);
    let mut undo = board.push(&Move::parse_from("e2e4"));
    undo(&mut board);

    loop {
        board.hash = generate_hash(&board);
        let mut input = String::new();
        let mut computer_move = false;
        println!("{:?}", generate_legal_moves(&mut board));

        io::stdin().read_line(&mut input).expect("Failed to read line");

        if &*input.trim() == "exit" {
            break;
        } else if &*input.trim() == "go" {
            computer_move = true;
        } else if &*input.trim() == "perft" {
            let start = Instant::now();
            println!("{}", perft::go(5, &mut board, 0));
            println!("finished in {:.2?}", start.elapsed());
        } else if &*input.trim() == "undo" {
            undo(&mut board);
            println!("{}", board);
            println!("{:#066b}", board.hash);
        } else {
            undo = board.push(&Move::parse_from(&*input.trim()));
            println!("{}", board);
            println!("{:#066b}", board.hash);
        }   
        
        if computer_move {
            let mut eval_result = EvalResult{best_move: Move{from: 0, to: 0, promotion: PieceTypes::Empty}};

            let start = Instant::now();
            // search(4, &mut board, &mut eval_result, 0);
            println!("{}", absearch(16, &mut board, search::REALLY_SMALL_NUMBER * 10.0, search::REALLY_BIG_NUMBER * 10.0, &mut eval_result, 0, &mut Vec::new()));
            let elapsed = start.elapsed();
            undo = board.push(&eval_result.best_move);

            println!("{}finished in {:.2?}", board, elapsed);
            println!("{:#066b}", board.hash);
            println!("{}", eval_result.best_move.uci());
        }
    }
    Ok(())
}