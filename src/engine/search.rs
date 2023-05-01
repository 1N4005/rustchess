use crate::game::{board::{Board, Move}, movegen::{generate_legal_moves, is_check}};
use super::eval::evaluate;

pub const REALLY_SMALL_NUMBER: f32 = -1000000.0;
pub const REALLY_BIG_NUMBER: f32 = 1000000.0;

pub struct EvalResult {
    pub best_move: Move
}

pub fn absearch(depth: u8, board: &mut Board, mut alpha: f32, beta: f32, mut eval_result: &mut EvalResult, depth_from_root: u8) -> f32{
    if depth == 0 {
        return evaluate(board);
    }

    let legal_moves = generate_legal_moves(board);
    
    if legal_moves.len() == 0 {
        if is_check(board, if board.wtomove {board.wkingpos} else {board.bkingpos}, board.wtomove) {
            return REALLY_SMALL_NUMBER - depth as f32;
        }
        return 0.0;
    }

    for m in legal_moves {
        let undo = board.push(&m);
        let eval = -absearch(depth - 1, board, -beta, -alpha, &mut eval_result, depth_from_root + 1);

        if eval > alpha {
            alpha = eval;
            if depth_from_root == 0 {
                eval_result.best_move = m.clone();
            }
        }

        if eval >= beta {
            undo(board);
            return beta;
        }

        undo(board);
    }

    alpha
}

pub fn search(depth: u8, board: &mut Board, eval_result: &mut EvalResult, depth_from_root: u8) -> f32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut best_eval = REALLY_SMALL_NUMBER;
    let legal_moves = generate_legal_moves(board);
    
    if legal_moves.len() == 0 {
        if is_check(board, if board.wtomove {board.wkingpos} else {board.bkingpos}, board.wtomove) {
            return REALLY_SMALL_NUMBER - depth as f32;
        }
        return 0.0;
    }

    for m in legal_moves {
        let undo = board.push(&m);
        let eval = -search(depth - 1, board, eval_result, depth_from_root + 1);
        if eval > best_eval {
            best_eval = eval;
            if depth_from_root == 0 {
                eval_result.best_move = m.clone();
            }
        }
        undo(board);
    }

    best_eval
}