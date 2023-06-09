use crate::game::{
    board::{Board, Move, PieceTypes, TranspositionEntry},
    movegen::{generate_legal_captures, generate_legal_moves, is_check},
};

use super::eval::evaluate;

pub const REALLY_SMALL_NUMBER: f32 = -1000.0;
pub const REALLY_BIG_NUMBER: f32 = 1000.0;

pub struct EvalResult {
    pub best_move: Move,
}

pub fn get_value(piece: PieceTypes) -> i16 {
    return match piece {
        PieceTypes::Pawn => 1,
        PieceTypes::Bishop => 3,
        PieceTypes::Knight => 3,
        PieceTypes::Rook => 5,
        PieceTypes::Queen => 9,
        PieceTypes::King => 100,
        _ => 0,
    };
}

pub fn order_moves(moves: &mut Vec<Move>, board: &Board) {
    fn guess_move(m: &Move, board: &Board) -> i16 {
        if board.board[m.to as usize].piece_type != PieceTypes::Empty {
            return get_value(board.board[m.to as usize].piece_type)
                - get_value(board.board[m.from as usize].piece_type);
        }
        0
    }
    moves.sort_by_key(|x| guess_move(x, board));
    moves.reverse();
}

pub fn absearch(
    depth: u8,
    board: &mut Board,
    mut alpha: f32,
    beta: f32,
    mut eval_result: &mut EvalResult,
    depth_from_root: u8,
    line: &mut Vec<Move>
) -> f32 {
    println!("{:?}, {}", line, depth_from_root);
    if depth_from_root > 0 {
        let entry = board.transposition_table.get(&board.hash);
        match entry {
            Some(&entry_unwrap) => {
                if entry_unwrap.depth >= depth {
                    println!("t found: {:?} with hash {:#066b}, depth {}, eval: {}", line, board.hash, depth_from_root, entry_unwrap.eval);
                    return entry_unwrap.eval;
                }
            }
            None => (),
        }
    }
    if depth == 0 {
        let eval = search_captures(board, alpha, beta);
        println!("final position: {:?}: {}", line, eval);
        return eval;
        // return evaluate(board);
    }

    let mut legal_moves = generate_legal_moves(board);
    order_moves(&mut legal_moves, board);

    if legal_moves.len() == 0 {
        if is_check(
            board,
            if board.wtomove {
                board.wkingpos
            } else {
                board.bkingpos
            },
            board.wtomove,
        ) {
            println!("final position: {:?}: mate", line);
            return REALLY_SMALL_NUMBER + depth_from_root as f32;
        }
        return 0.0;
    }

    for m in &legal_moves {
        let undo = board.push(&m);
        line.push(*m);
        let eval = -absearch(
            depth - 1,
            board,
            -beta,
            -alpha,
            &mut eval_result,
            depth_from_root + 1,
            line
        );
        line.pop();
    
        undo(board);

        if eval >= beta {
            println!("saving eval {} with line {:?} in position {:#066b}", beta, line, board.hash);
            board.transposition_table.insert(
                board.hash,
                TranspositionEntry {
                    depth: depth,
                    eval: beta,
                },
            );
            return beta;
        }

        if eval > alpha {
            let prev_eval = eval;
            alpha = eval;
            println!("alpha: {}, eval: {}, depth_from_root: {}", alpha, prev_eval, depth_from_root);
            if depth_from_root == 0 {
                println!("alpha: {}, eval: {}", alpha, prev_eval);
                eval_result.best_move = m.clone();
            }
        }
    }

    println!("saving eval {} with line {:?} in position {:#066b}", alpha, line, board.hash);
    board.transposition_table.insert(
        board.hash,
        TranspositionEntry {
            depth: depth,
            eval: alpha,
        },
    );

    alpha
}

pub fn search_captures(board: &mut Board, mut alpha: f32, mut beta: f32) -> f32 {
    let mut legal_captures = generate_legal_captures(board);
    order_moves(&mut legal_captures, board);

    if legal_captures.len() == 0 {
        return evaluate(board);
    }

    for m in legal_captures {
        let undo = board.push(&m);

        let eval = -search_captures(board, -beta, -alpha);

        undo(board);

        if eval >= beta {
            return beta;
        }

        if eval > alpha {
            alpha = eval;
        }
    }

    alpha
}
