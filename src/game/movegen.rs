use super::board::{self, Board, Move, Piece, PieceTypes};

pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut curr_square: u8 = 0;
    for piece in board.board {
        match piece.piece_type {
            board::PieceTypes::Empty => {}
            board::PieceTypes::Pawn => {
                pawn_moves(&mut moves, board, &piece, curr_square);
                pawn_captures(&mut moves, board, &piece, curr_square);
            }
            board::PieceTypes::Bishop => {}
            board::PieceTypes::Knight => {}
            board::PieceTypes::Rook => {}
            board::PieceTypes::Queen => {}
            board::PieceTypes::King => {}
        }
        curr_square += 1;
    }
    moves
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

        if curr_square - 7 == board.enpassant_square {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 7,
                promotion: PieceTypes::Empty,
            });
        }

        if curr_square - 9 == board.enpassant_square {
            moves.push(Move {
                from: curr_square,
                to: curr_square - 9,
                promotion: PieceTypes::Empty,
            });
        }
    } else if !piece.white && !board.wtomove {
        if curr_square % 8 == 0
            && ((board.board[curr_square as usize + 7].white
                && board.board[curr_square as usize + 7].piece_type != PieceTypes::Empty)
                || curr_square + 7 == board.enpassant_square)
        {
            if curr_square / 8 == 1 {
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
        if curr_square % 8 == 7
            && ((board.board[curr_square as usize + 9].white
                && board.board[curr_square as usize + 9].piece_type != PieceTypes::Empty)
                || curr_square + 9 == board.enpassant_square)
        {
            if curr_square / 8 == 1 {
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

        if board.board[curr_square as usize + 9].white
            && board.board[curr_square as usize + 9].piece_type != PieceTypes::Empty
        {
            if curr_square / 8 == 1 {
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
        if board.board[curr_square as usize + 7].white
            && board.board[curr_square as usize + 7].piece_type != PieceTypes::Empty
        {
            if curr_square / 8 == 1 {
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

        if curr_square + 7 == board.enpassant_square {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 7,
                promotion: PieceTypes::Empty,
            });
        }

        if curr_square + 9 == board.enpassant_square {
            moves.push(Move {
                from: curr_square,
                to: curr_square + 9,
                promotion: PieceTypes::Empty,
            });
        }
    }
}
