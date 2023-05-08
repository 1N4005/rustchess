use crate::game::board::{parse_square, SQUARES};

use super::board::{self, Board, Move, Piece, PieceTypes};
use std::cmp;

const NORTH: u8 = 0b00000001;
const EAST: u8 = 0b00000010;
const SOUTH: u8 = 0b00000100;
const WEST: u8 = 0b00001000;

const NE: u8 = 0b00010000;
const NW: u8 = 0b00100000;
const SE: u8 = 0b01000000;
const SW: u8 = 0b10000000;

const HORIZONTAL: u8 = 0b00001111;
const VERTICAL: u8 = 0b11110000;

#[derive(Clone, Copy, Debug)]
pub struct MoveData {
    n: u8,
    e: u8,
    s: u8,
    w: u8,

    ne: u8,
    nw: u8,
    se: u8,
    sw: u8,
}

pub fn compute_distances() -> [MoveData; 64] {
    let mut distances = [MoveData {
        n: 0,
        e: 0,
        s: 0,
        w: 0,
        ne: 0,
        nw: 0,
        se: 0,
        sw: 0,
    }; 64];

    let mut i: u8 = 0;
    for _square in distances {
        let rank = i / 8;
        let file = i % 8;

        distances[i as usize].n = rank;
        distances[i as usize].s = 7 - rank;
        distances[i as usize].w = file;
        distances[i as usize].e = 7 - file;

        distances[i as usize].ne = cmp::min(distances[i as usize].n, distances[i as usize].e);
        distances[i as usize].nw = cmp::min(distances[i as usize].n, distances[i as usize].w);
        distances[i as usize].se = cmp::min(distances[i as usize].s, distances[i as usize].e);
        distances[i as usize].sw = cmp::min(distances[i as usize].s, distances[i as usize].w);

        i += 1;
    }

    distances
}

pub fn generate_pseudolegal_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut curr_square: u8 = 0;
    for piece in board.board {
        match piece.piece_type {
            board::PieceTypes::Empty => {}
            board::PieceTypes::Pawn => {
                pawn_moves(&mut moves, board, &piece, curr_square);
                pawn_captures(&mut moves, board, &piece, curr_square);
            }
            board::PieceTypes::Bishop => {
                bishop_moves(&mut moves, board, &piece, curr_square);
            }
            board::PieceTypes::Knight => {
                knight_moves(&mut moves, board, &piece, curr_square);
            }
            board::PieceTypes::Rook => {
                rook_moves(&mut moves, board, &piece, curr_square);
            }
            board::PieceTypes::Queen => {
                queen_moves(&mut moves, board, &piece, curr_square);
            }
            board::PieceTypes::King => {
                king_moves(&mut moves, board, &piece, curr_square);
            }
        }
        curr_square += 1;
    }
    moves
}

pub fn generate_legal_moves(board: &mut Board) -> Vec<Move> {
    let pseudo_moves = generate_pseudolegal_moves(board);
    let mut legal_moves: Vec<Move> = Vec::new();
    for m in pseudo_moves {
        // println!("{}", m.uci());
        let undo = board.push(&m);

        if board.wtomove {
            board.wtomove = !board.wtomove;
            if !is_check(board, board.bkingpos, false) {
                legal_moves.push(m);
            }
            board.wtomove = !board.wtomove;
        } else {
            board.wtomove = !board.wtomove;
            if !is_check(board, board.wkingpos, true) {
                legal_moves.push(m);
            }
            board.wtomove = !board.wtomove;
        }

        board.wtomove = !board.wtomove;
        undo(board);
    }

    legal_moves
}

pub fn generate_legal_captures(board: &mut Board) -> Vec<Move> {
    let pseudo_moves = generate_pseudolegal_moves(board);
    let mut legal_moves: Vec<Move> = Vec::new();

    for m in pseudo_moves {
        // println!("{}", m.uci());
        let move_to = board.board[m.to as usize].piece_type;
        let undo = board.push(&m);
        if move_to != PieceTypes::Empty {
            if board.wtomove {
                board.wtomove = !board.wtomove;
                if !is_check(board, board.bkingpos, false) {
                    legal_moves.push(m);
                }
                board.wtomove = !board.wtomove;
            } else {
                board.wtomove = !board.wtomove;
                if !is_check(board, board.wkingpos, true) {
                    legal_moves.push(m);
                }
                board.wtomove = !board.wtomove;
            }
        }

        board.wtomove = !board.wtomove;
        undo(board);
    }

    legal_moves
}

fn pawn_moves(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    if piece.white
        && board.wtomove
        && board.board[curr_square as usize - 8].piece_type == PieceTypes::Empty
    {
        if curr_square / 8 == 6 {
            if board.board[curr_square as usize - 16].piece_type == PieceTypes::Empty {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 16,
                    promotion: PieceTypes::Empty,
                });
            }
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Empty,
            });
        } else if curr_square / 8 == 1 {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Knight,
            });
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Bishop,
            });
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Rook,
            });
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Queen,
            });
        } else {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Empty,
            });
        }
    } else if !piece.white
        && !board.wtomove
        && board.board[curr_square as usize + 8].piece_type == PieceTypes::Empty
    {
        if curr_square / 8 == 1 {
            if board.board[curr_square as usize + 16].piece_type == PieceTypes::Empty {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 16,
                    promotion: PieceTypes::Empty,
                });
            }
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Empty,
            });
        } else if curr_square / 8 == 6 {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Knight,
            });
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Bishop,
            });
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Rook,
            });
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Queen,
            });
        } else {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Empty,
            });
        }
    }
}

fn pawn_captures(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    if piece.white && board.wtomove {
        if curr_square % 8 == 0
            && ((board.board[curr_square as usize - 7].white == false
                && board.board[curr_square as usize - 7].piece_type != PieceTypes::Empty)
                || curr_square - 7 == board.enpassant_square)
        {
            if curr_square / 8 == 1 {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 7,
                    promotion: PieceTypes::Knight,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 7,
                    promotion: PieceTypes::Bishop,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 7,
                    promotion: PieceTypes::Rook,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 7,
                    promotion: PieceTypes::Queen,
                });
            } else {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 7,
                    promotion: PieceTypes::Empty,
                });
            }
            return;
        }
        if curr_square % 8 == 7
            && ((board.board[curr_square as usize - 9].white == false
                && board.board[curr_square as usize - 9].piece_type != PieceTypes::Empty)
                || curr_square - 9 == board.enpassant_square)
        {
            if curr_square / 8 == 1 {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 9,
                    promotion: PieceTypes::Knight,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 9,
                    promotion: PieceTypes::Bishop,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 9,
                    promotion: PieceTypes::Rook,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 9,
                    promotion: PieceTypes::Queen,
                });
            } else {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 9,
                    promotion: PieceTypes::Empty,
                });
            }
            return;
        }
        if curr_square % 8 != 0 {
            if board.board[curr_square as usize - 9].white == false
                && board.board[curr_square as usize - 9].piece_type != PieceTypes::Empty
            {
                if curr_square / 8 == 1 {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 9,
                        promotion: PieceTypes::Knight,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 9,
                        promotion: PieceTypes::Bishop,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 9,
                        promotion: PieceTypes::Rook,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 9,
                        promotion: PieceTypes::Queen,
                    });
                } else {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 9,
                        promotion: PieceTypes::Empty,
                    });
                }
            }
        }
        if curr_square % 8 != 7 {
            if board.board[curr_square as usize - 7].white == false
                && board.board[curr_square as usize - 7].piece_type != PieceTypes::Empty
            {
                if curr_square / 8 == 1 {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 7,
                        promotion: PieceTypes::Knight,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 7,
                        promotion: PieceTypes::Bishop,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 7,
                        promotion: PieceTypes::Rook,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 7,
                        promotion: PieceTypes::Queen,
                    });
                } else {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - 7,
                        promotion: PieceTypes::Empty,
                    });
                }
            }
        }
        if curr_square % 8 != 7 {
            if curr_square - 7 == board.enpassant_square {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 7,
                    promotion: PieceTypes::Empty,
                });
            }
        }
        if curr_square % 8 != 0 {
            if curr_square - 9 == board.enpassant_square {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 9,
                    promotion: PieceTypes::Empty,
                });
            }
        }
    } else if !piece.white && !board.wtomove {
        if curr_square % 8 == 0
            && ((board.board[curr_square as usize + 9].white
                && board.board[curr_square as usize + 9].piece_type != PieceTypes::Empty)
                || curr_square + 9 == board.enpassant_square)
        {
            if curr_square / 8 == 6 {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 9,
                    promotion: PieceTypes::Knight,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 9,
                    promotion: PieceTypes::Bishop,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 9,
                    promotion: PieceTypes::Rook,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 9,
                    promotion: PieceTypes::Queen,
                });
            } else {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 9,
                    promotion: PieceTypes::Empty,
                });
            }
            return;
        }
        if curr_square % 8 == 7
            && ((board.board[curr_square as usize + 7].white
                && board.board[curr_square as usize + 7].piece_type != PieceTypes::Empty)
                || curr_square + 7 == board.enpassant_square)
        {
            if curr_square / 8 == 6 {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 7,
                    promotion: PieceTypes::Knight,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 7,
                    promotion: PieceTypes::Bishop,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 7,
                    promotion: PieceTypes::Rook,
                });
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 7,
                    promotion: PieceTypes::Queen,
                });
            } else {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 7,
                    promotion: PieceTypes::Empty,
                });
            }
            return;
        }
        if curr_square % 8 != 7 {
            if board.board[curr_square as usize + 9].white
                && board.board[curr_square as usize + 9].piece_type != PieceTypes::Empty
            {
                if curr_square / 8 == 6 {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 9,
                        promotion: PieceTypes::Knight,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 9,
                        promotion: PieceTypes::Bishop,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 9,
                        promotion: PieceTypes::Rook,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 9,
                        promotion: PieceTypes::Queen,
                    });
                } else {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 9,
                        promotion: PieceTypes::Empty,
                    });
                }
            }
        }
        if curr_square % 8 != 0 {
            if board.board[curr_square as usize + 7].white
                && board.board[curr_square as usize + 7].piece_type != PieceTypes::Empty
            {
                if curr_square / 8 == 6 {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 7,
                        promotion: PieceTypes::Knight,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 7,
                        promotion: PieceTypes::Bishop,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 7,
                        promotion: PieceTypes::Rook,
                    });
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 7,
                        promotion: PieceTypes::Queen,
                    });
                } else {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + 7,
                        promotion: PieceTypes::Empty,
                    });
                }
            }
        }
        if curr_square % 8 != 0 {
            if curr_square + 7 == board.enpassant_square {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 7,
                    promotion: PieceTypes::Empty,
                });
            }
        }
        if curr_square % 8 != 7 {
            if curr_square + 9 == board.enpassant_square {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 9,
                    promotion: PieceTypes::Empty,
                });
            }
        }
    }
}

fn knight_moves(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    let rank = curr_square / 8;
    let file = curr_square % 8;
    if rank > 1 {
        if file > 0
            && (board.board[curr_square as usize - 17].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize - 17].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 17,
                promotion: PieceTypes::Empty,
            });
        }
        if file < 7
            && (board.board[curr_square as usize - 15].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize - 15].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 15,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if rank < 6 {
        if file > 0
            && (board.board[curr_square as usize + 15].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize + 15].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 15,
                promotion: PieceTypes::Empty,
            });
        }
        if file < 7
            && (board.board[curr_square as usize + 17].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize + 17].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 17,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if file > 1 {
        if rank > 0
            && (board.board[curr_square as usize - 10].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize - 10].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 10,
                promotion: PieceTypes::Empty,
            });
        }
        if rank < 7
            && (board.board[curr_square as usize + 6].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize + 6].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 6,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if file < 6 {
        if rank > 0
            && (board.board[curr_square as usize - 6].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize - 6].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 6,
                promotion: PieceTypes::Empty,
            });
        }
        if rank < 7
            && (board.board[curr_square as usize + 10].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize + 10].white != board.wtomove)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 10,
                promotion: PieceTypes::Empty,
            });
        }
    }
}

fn sliding_piece_moves(
    moves: &mut Vec<Move>,
    board: &Board,
    piece: &Piece,
    curr_square: u8,
    direction: u8,
) {
    if piece.white == board.wtomove {
        if direction & NORTH == NORTH {
            for i in 1..board.precomputed_move_data[curr_square as usize].n as usize + 1 {
                if board.board[curr_square as usize - i * 8].white != board.wtomove
                    && board.board[curr_square as usize - i * 8].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - i as u8 * 8,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize - i * 8].white == board.wtomove
                    && board.board[curr_square as usize - i * 8].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square - i as u8 * 8,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & SOUTH == SOUTH {
            for i in 1..board.precomputed_move_data[curr_square as usize].s as usize + 1 {
                if board.board[curr_square as usize + i * 8].white != board.wtomove
                    && board.board[curr_square as usize + i * 8].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + i as u8 * 8,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize + i * 8].white == board.wtomove
                    && board.board[curr_square as usize + i * 8].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square + i as u8 * 8,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & EAST == EAST {
            for i in 1..board.precomputed_move_data[curr_square as usize].e as usize + 1 {
                if board.board[curr_square as usize + i].white != board.wtomove
                    && board.board[curr_square as usize + i].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + i as u8,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize + i].white == board.wtomove
                    && board.board[curr_square as usize + i].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square + i as u8,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & WEST == WEST {
            for i in 1..board.precomputed_move_data[curr_square as usize].w as usize + 1 {
                if board.board[curr_square as usize - i].white != board.wtomove
                    && board.board[curr_square as usize - i].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - i as u8,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize - i].white == board.wtomove
                    && board.board[curr_square as usize - i].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square - i as u8,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & NE == NE {
            for i in 1..board.precomputed_move_data[curr_square as usize].ne as usize + 1 {
                if board.board[curr_square as usize - i * 7].white != board.wtomove
                    && board.board[curr_square as usize - i * 7].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - i as u8 * 7,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize - i * 7].white == board.wtomove
                    && board.board[curr_square as usize - i * 7].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square - i as u8 * 7,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & NW == NW {
            for i in 1..board.precomputed_move_data[curr_square as usize].nw as usize + 1 {
                if board.board[curr_square as usize - i * 9].white != board.wtomove
                    && board.board[curr_square as usize - i * 9].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square - i as u8 * 9,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize - i * 9].white == board.wtomove
                    && board.board[curr_square as usize - i * 9].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square - i as u8 * 9,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & SE == SE {
            for i in 1..board.precomputed_move_data[curr_square as usize].se as usize + 1 {
                if board.board[curr_square as usize + i * 9].white != board.wtomove
                    && board.board[curr_square as usize + i * 9].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + i as u8 * 9,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize + i * 9].white == board.wtomove
                    && board.board[curr_square as usize + i * 9].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square + i as u8 * 9,
                    promotion: PieceTypes::Empty,
                });
            }
        }

        if direction & SW == SW {
            for i in 1..board.precomputed_move_data[curr_square as usize].sw as usize + 1 {
                if board.board[curr_square as usize + i * 7].white != board.wtomove
                    && board.board[curr_square as usize + i * 7].piece_type != PieceTypes::Empty
                {
                    moves.push(Move {
                        from: curr_square,
                        to: curr_square + i as u8 * 7,
                        promotion: PieceTypes::Empty,
                    });
                    break;
                }

                if board.board[curr_square as usize + i * 7].white == board.wtomove
                    && board.board[curr_square as usize + i * 7].piece_type != PieceTypes::Empty
                {
                    break;
                }

                moves.push(Move {
                    from: curr_square,
                    to: curr_square + i as u8 * 7,
                    promotion: PieceTypes::Empty,
                });
            }
        }
    }
}

fn bishop_moves(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    sliding_piece_moves(moves, board, piece, curr_square, 0b11110000);
}

fn rook_moves(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    sliding_piece_moves(moves, board, piece, curr_square, 0b00001111);
}

fn queen_moves(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    sliding_piece_moves(moves, board, piece, curr_square, 0b11111111);
}

fn king_moves(moves: &mut Vec<Move>, board: &Board, piece: &Piece, curr_square: u8) {
    let rank = curr_square / 8;
    let file = curr_square % 8;

    if rank > 0 {
        if file > 0
            && (board.board[curr_square as usize - 9].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize - 9].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 9,
                promotion: PieceTypes::Empty,
            });
        }

        if file < 7
            && (board.board[curr_square as usize - 7].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize - 7].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 7,
                promotion: PieceTypes::Empty,
            });
        }

        if (board.board[curr_square as usize - 8].piece_type == PieceTypes::Empty
            || board.board[curr_square as usize - 8].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 8,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if rank < 7 {
        if file > 0
            && (board.board[curr_square as usize + 7].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize + 7].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 7,
                promotion: PieceTypes::Empty,
            });
        }

        if file < 7
            && (board.board[curr_square as usize + 9].piece_type == PieceTypes::Empty
                || board.board[curr_square as usize + 9].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 9,
                promotion: PieceTypes::Empty,
            });
        }

        if (board.board[curr_square as usize + 8].piece_type == PieceTypes::Empty
            || board.board[curr_square as usize + 8].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 8,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if file > 0 {
        if (board.board[curr_square as usize - 1].piece_type == PieceTypes::Empty
            || board.board[curr_square as usize - 1].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 1,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if file < 7 {
        if (board.board[curr_square as usize + 1].piece_type == PieceTypes::Empty
            || board.board[curr_square as usize + 1].white != piece.white)
            && piece.white == board.wtomove
        {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 1,
                promotion: PieceTypes::Empty,
            });
        }
    }

    if piece.white && board.wtomove {
        if board.wkingpos == 60 {
            if board.board[curr_square as usize + 1].piece_type == PieceTypes::Empty
                && board.board[curr_square as usize + 2].piece_type == PieceTypes::Empty
                && board.white_ks
                && !is_check(board, curr_square + 1, true)
                && !is_check(board, curr_square, true)
            {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 2,
                    promotion: PieceTypes::Empty,
                });
            }
            if board.board[curr_square as usize - 1].piece_type == PieceTypes::Empty
                && board.board[curr_square as usize - 2].piece_type == PieceTypes::Empty
                && board.board[curr_square as usize - 3].piece_type == PieceTypes::Empty
                && board.white_qs
                && !is_check(board, curr_square - 1, true)
                && !is_check(board, curr_square, true)
            {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 2,
                    promotion: PieceTypes::Empty,
                });
            }
        }
    } else if !piece.white && !board.wtomove {
        if board.bkingpos == 4 {
            if board.board[curr_square as usize + 1].piece_type == PieceTypes::Empty
                && board.board[curr_square as usize + 2].piece_type == PieceTypes::Empty
                && board.black_ks
                && !is_check(board, curr_square + 1, false)
                && !is_check(board, curr_square, false)
            {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square + 2,
                    promotion: PieceTypes::Empty,
                });
            }
            if board.board[curr_square as usize - 1].piece_type == PieceTypes::Empty
                && board.board[curr_square as usize - 2].piece_type == PieceTypes::Empty
                && board.board[curr_square as usize - 3].piece_type == PieceTypes::Empty
                && board.black_qs
                && !is_check(board, curr_square - 1, false)
                && !is_check(board, curr_square, false)
            {
                moves.push(Move {
                    from: curr_square,
                    to: curr_square - 2,
                    promotion: PieceTypes::Empty,
                });
            }
        }
    }
}

pub fn is_check(board: &Board, curr_square: u8, white: bool) -> bool {
    let piece = Piece {
        piece_type: PieceTypes::Empty,
        white,
    };

    let mut moves: Vec<Move> = Vec::new();
    let mut dmoves: Vec<Move> = Vec::new();
    let mut hmoves: Vec<Move> = Vec::new();
    let mut pcaptures: Vec<Move> = Vec::new();
    let mut kmoves: Vec<Move> = Vec::new();

    sliding_piece_moves(&mut dmoves, board, &piece, curr_square, 0b11110000);
    sliding_piece_moves(&mut hmoves, board, &piece, curr_square, 0b00001111);
    if white && curr_square / 8 > 0{
        pawn_captures(&mut pcaptures, board, &piece, curr_square);
    } else if !white && curr_square / 8 < 7{
        pawn_captures(&mut pcaptures, board, &piece, curr_square);
    }
    knight_moves(&mut moves, board, &piece, curr_square);
    king_moves(&mut kmoves, board, &piece, curr_square);

    for m in moves {
        let piece = board.board[m.to as usize];
        if piece.piece_type == PieceTypes::Knight && piece.white != white {
            return true;
        }
    }

    for m in dmoves {
        let piece = board.board[m.to as usize];
        if (piece.piece_type == PieceTypes::Bishop || piece.piece_type == PieceTypes::Queen)
            && piece.white != white
        {
            return true;
        }
    }

    for m in hmoves {
        let piece = board.board[m.to as usize];
        if (piece.piece_type == PieceTypes::Rook || piece.piece_type == PieceTypes::Queen)
            && piece.white != white
        {
            return true;
        }
    }

    for m in pcaptures {
        let piece = board.board[m.to as usize];
        if piece.piece_type == PieceTypes::Pawn
            && piece.white != white
            && m.to != board.enpassant_square
        {
            return true;
        }
    }

    for m in kmoves {
        let piece = board.board[m.to as usize];
        if piece.piece_type == PieceTypes::King {
            return true;
        }
    }

    return false;
}
