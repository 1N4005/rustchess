use rand::prelude::random;

use crate::game::board::Board;

pub const WPAWN: usize = 0;
pub const BPAWN: usize = 1;
pub const WBISHOP: usize = 2;
pub const BBISHOP: usize = 3;
pub const WKNIGHT: usize = 4;
pub const BKNIGHT: usize = 5;
pub const WROOK: usize = 6;
pub const BROOK: usize = 7;
pub const WQUEEN: usize = 8;
pub const BQUEEN: usize = 9;
pub const WKING: usize = 10;
pub const BKING: usize = 11;

pub fn init_table() -> [[u64; 12]; 64] {
    let mut table = [[0u64; 12]; 64];
    for i in 0..64 {
        for j in 0..12 {
            table[i][j] = random();
        }
    }

    table
}


pub fn generate_hash(board: &Board) -> u64 {
    let mut hash = 0u64;
    let mut curr_square: usize = 0;
    for piece in board.board {
        match piece.piece_type {
            crate::game::board::PieceTypes::Empty => (),
            crate::game::board::PieceTypes::Pawn => if piece.white {
                hash ^= board.zobrist_table[curr_square][WPAWN];
            } else {
                hash ^= board.zobrist_table[curr_square][BPAWN];
            },
            crate::game::board::PieceTypes::Bishop => if piece.white {
                hash ^= board.zobrist_table[curr_square][WBISHOP];
            } else {
                hash ^= board.zobrist_table[curr_square][BBISHOP];
            },
            crate::game::board::PieceTypes::Knight => if piece.white {
                hash ^= board.zobrist_table[curr_square][WKNIGHT];
            } else {
                hash ^= board.zobrist_table[curr_square][BKNIGHT];
            },
            crate::game::board::PieceTypes::Rook => if piece.white {
                hash ^= board.zobrist_table[curr_square][WROOK];
            } else {
                hash ^= board.zobrist_table[curr_square][BROOK];
            },
            crate::game::board::PieceTypes::Queen => if piece.white {
                hash ^= board.zobrist_table[curr_square][WQUEEN];
            } else {
                hash ^= board.zobrist_table[curr_square][BQUEEN];
            },
            crate::game::board::PieceTypes::King => if piece.white {
                hash ^= board.zobrist_table[curr_square][WKING];
            } else {
                hash ^= board.zobrist_table[curr_square][BKING];
            },
        }
        curr_square += 1;
    }

    hash
}