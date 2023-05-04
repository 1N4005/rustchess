use crate::game::board::{Board, PieceTypes};
use super::piecemaps;

pub fn evaluate(board: &Board) -> f32 {
    let mut eval: f32 = 0.0;

    let mut square: usize = 0;
    for piece in board.board {
        eval += match piece.piece_type {        
            PieceTypes::Pawn => {if piece.white {1.0 * piecemaps::WHITE_PAWN_MAP[square]} else {-1.0 * piecemaps::BLACK_PAWN_MAP[square]}},
            PieceTypes::Bishop => {if piece.white {3.0 * piecemaps::KNIGHT_MAP[square]} else {-3.0 * piecemaps::KNIGHT_MAP[square]}},
            PieceTypes::Knight => {if piece.white {3.0} else {-3.0}},
            PieceTypes::Rook => {if piece.white {5.0} else {-5.0}},
            PieceTypes::Queen => {if piece.white {9.0} else {-9.0}},
            _ => {0.0},
        };
        square += 1;
    }

    eval * if board.wtomove {1.0} else {-1.0}
}