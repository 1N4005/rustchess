use crate::game::board::{Board, PieceTypes};

pub fn evaluate(board: &Board) -> f32 {
    let mut eval: f32 = 0.0;

    for piece in board.board {
        eval += match piece.piece_type {        
            PieceTypes::Pawn => {if piece.white {1.0} else {-1.0}},
            PieceTypes::Bishop => {if piece.white {3.0} else {-3.0}},
            PieceTypes::Knight => {if piece.white {3.0} else {-3.0}},
            PieceTypes::Rook => {if piece.white {5.0} else {-5.0}},
            PieceTypes::Queen => {if piece.white {9.0} else {-9.0}},
            _ => {0.0},
        }
    }

    eval * if board.wtomove {1.0} else {-1.0}
}