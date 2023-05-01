use std::fmt::Display;

use super::movegen::{MoveData, compute_distances};

pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const SQUARES: [&str; 64] = [
    "a8","b8","c8","d8","e8","f8","g8","h8",
    "a7","b7","c7","d7","e7","f7","g7","h7",
    "a6","b6","c6","d6","e6","f6","g6","h6",
    "a5","b5","c5","d5","e5","f5","g5","h5",
    "a4","b4","c4","d4","e4","f4","g4","h4",
    "a3","b3","c3","d3","e3","f3","g3","h3",
    "a2","b2","c2","d2","e2","f2","g2","h2",
    "a1","b1","c1","d1","e1","f1","g1","h1",
    ];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceTypes {
    Empty,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: PieceTypes,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub board: [Piece; 64],

    pub wtomove: bool,

    pub white_ks: bool,
    pub white_qs: bool,
    pub black_ks: bool,
    pub black_qs: bool,
    pub enpassant_square: u8,
    pub fullmoves: u16,
    pub bkingpos: u8,
    pub wkingpos: u8,

    pub precomputed_move_data: [MoveData; 64],
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub white: bool,
}

impl Move {
    pub fn uci(&self) -> String {
        format!("{}{}{}", SQUARES[self.from as usize], SQUARES[self.to as usize], match self.promotion {
            PieceTypes::Bishop => "b",
            PieceTypes::Knight => "n",
            PieceTypes::Rook => "r",
            PieceTypes::Queen => "q",
            _ => ""
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i: u8 = 0;
        for piece in self.board {
            match piece.piece_type {
                PieceTypes::Empty => write!(f, "-"),
                PieceTypes::Pawn => write!(f, "{}", if piece.white { "P" } else { "p" }),
                PieceTypes::Bishop => write!(f, "{}", if piece.white { "B" } else { "b" }),
                PieceTypes::Knight => write!(f, "{}", if piece.white { "N" } else { "n" }),
                PieceTypes::Rook => write!(f, "{}", if piece.white { "R" } else { "r" }),
                PieceTypes::Queen => write!(f, "{}", if piece.white { "Q" } else { "q" }),
                PieceTypes::King => write!(f, "{}", if piece.white { "K" } else { "k" }),
            }?;

            if i % 8 == 7 {
                println!();
            }
            i += 1;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(fen: &str) -> Self {
        let fen_array = fen.split(" ");
        let mut tokens: [&str; 6] = [""; 6];

        let mut i = 0;

        for token in fen_array {
            tokens[i] = token;
            i += 1;
        }

        let board_array = tokens[0].split("/");

        let mut board: [Piece; 64] = [Piece {
            piece_type: PieceTypes::Empty,
            white: false,
        }; 64];
        let mut white_ks: bool = false;
        let mut white_qs: bool = false;
        let mut black_ks: bool = false;
        let mut black_qs: bool = false;
        let wtomove: bool = tokens[1] == "w";

        let mut curr_square: usize = 0;

        let mut bkingpos: u8 = 64;
        let mut wkingpos: u8 = 64;

        for row in board_array {
            for square in row.chars() {
                if square == '1' {
                    curr_square += 1
                } else if square == '2' {
                    curr_square += 2
                } else if square == '3' {
                    curr_square += 3
                } else if square == '4' {
                    curr_square += 4
                } else if square == '5' {
                    curr_square += 5
                } else if square == '6' {
                    curr_square += 6
                } else if square == '7' {
                    curr_square += 7
                } else if square == '8' {
                    curr_square += 8
                } else {
                    if square == 'p' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Pawn,
                            white: false,
                        };
                    } else if square == 'b' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Bishop,
                            white: false,
                        };
                    } else if square == 'n' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Knight,
                            white: false,
                        };
                    } else if square == 'r' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Rook,
                            white: false,
                        };
                    } else if square == 'q' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Queen,
                            white: false,
                        };
                    } else if square == 'k' {
                        bkingpos = curr_square as u8;
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::King,
                            white: false,
                        };
                    } else if square == 'P' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Pawn,
                            white: true,
                        };
                    } else if square == 'B' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Bishop,
                            white: true,
                        };
                    } else if square == 'N' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Knight,
                            white: true,
                        };
                    } else if square == 'R' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Rook,
                            white: true,
                        };
                    } else if square == 'Q' {
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::Queen,
                            white: true,
                        };
                    } else if square == 'K' {
                        wkingpos = curr_square as u8;
                        board[curr_square] = Piece {
                            piece_type: PieceTypes::King,
                            white: true,
                        };
                    } else {
                        panic!("invalid fen: {}", square);
                    }
                    curr_square += 1
                }
            }
        }

        if tokens[2].contains('K') {
            white_ks = true;
        }

        if tokens[2].contains('Q') {
            white_qs = true;
        }

        if tokens[2].contains('k') {
            black_ks = true;
        }

        if tokens[2].contains('q') {
            black_qs = true;
        }

        let enpassant_square;

        if tokens[3] == "-" {
            enpassant_square = 64;
        } else {
            enpassant_square = parse_square(tokens[3]);
        }

        let fullmoves = tokens[5].parse().unwrap();

        Board {
            board: board,
            wtomove: wtomove,
            white_ks: white_ks,
            white_qs: white_qs,
            black_ks: black_ks,
            black_qs: black_qs,
            enpassant_square: enpassant_square,
            fullmoves: fullmoves,
            precomputed_move_data: compute_distances(),
            bkingpos: bkingpos,
            wkingpos: wkingpos,
        }
    }

    pub fn push(&mut self, m: &Move) -> impl Fn(&mut Board){
        // assert_ne!(self.board[m.from as usize].piece_type, PieceTypes::Empty);

        let uboard = self.board;
        let uwtomove = self.wtomove;
        let uwhite_ks = self.white_ks;
        let uwhite_qs = self.white_qs;
        let ublack_ks = self.black_ks;
        let ublack_qs = self.black_qs;
        let uenpassant_square = self.enpassant_square;
        let ufullmoves = self.fullmoves;
        let uprecomputed_move_data = self.precomputed_move_data;
        let ubkingpos = self.bkingpos;
        let uwkingpos = self.wkingpos;

        let temp: Piece;
        if m.promotion == PieceTypes::Empty {
            temp = self.board[m.from as usize];
        } else {
            temp = Piece {
                piece_type: m.promotion,
                white: self.board[m.from as usize].white,
            }
        }    

        if self.board[m.from as usize].piece_type == PieceTypes::King {
            if self.wtomove {
                self.white_ks = false;
                self.white_qs = false;
            } else {
                self.black_ks = false;
                self.black_qs = false;
            }
        }


        if m.to == 63 {
            self.white_ks = false;
        } else if m.to == 56 {
            self.white_qs = false;
        } else if m.to == 0 {
            self.black_qs = false;
        } else if m.to == 7 {
            self.black_ks = false;
        }

        if m.from == 63 {
            self.white_ks = false;
        } else if m.from == 56 {
            self.white_qs = false;
        } else if m.from == 0 {
            self.black_qs = false;
        } else if m.from == 7 {
            self.black_ks = false;
        }

        if self.board[m.from as usize].piece_type == PieceTypes::Pawn {
            

            if m.to == self.enpassant_square {
                if self.wtomove {
                    self.board[(m.to + 8) as usize] = Piece {
                        piece_type: PieceTypes::Empty,
                        white: false,
                    };
                } else {
                    self.board[(m.to - 8) as usize] = Piece {
                        piece_type: PieceTypes::Empty,
                        white: false,
                    };
                }
            }
            
            if self.wtomove && m.from - m.to == 16 {
                self.enpassant_square = m.from - 8;
            } else if !self.wtomove && m.to - m.from == 16 {
                self.enpassant_square = m.to - 8;
            } else {
                self.enpassant_square = 64;
            }
        } else {
            self.enpassant_square = 64;
        }

        if self.board[m.from as usize].piece_type == PieceTypes::King {
            if self.board[m.from as usize].white {
                self.wkingpos = m.to as u8;
            } else {
                self.bkingpos = m.to as u8;
            }
        }

        if self.board[m.from as usize].piece_type == PieceTypes::King
            && m.from == 60
            && self.board[m.from as usize].white
        {
            if m.to == 62 {
                self.board[63] = Piece {
                    piece_type: PieceTypes::Empty,
                    white: true,
                };
                self.board[61] = Piece {
                    piece_type: PieceTypes::Rook,
                    white: true,
                };
            } else if m.to == 58 {
                self.board[56] = Piece {
                    piece_type: PieceTypes::Empty,
                    white: true,
                };
                self.board[59] = Piece {
                    piece_type: PieceTypes::Rook,
                    white: true,
                };
            }
        } else if self.board[m.from as usize].piece_type == PieceTypes::King
            && m.from == 4
            && !self.board[m.from as usize].white
        {
            if m.to == 6 {
                self.board[7] = Piece {
                    piece_type: PieceTypes::Empty,
                    white: false,
                };
                self.board[5] = Piece {
                    piece_type: PieceTypes::Rook,
                    white: false,
                };
            } else if m.to == 2 {
                self.board[0] = Piece {
                    piece_type: PieceTypes::Empty,
                    white: false,
                };
                self.board[3] = Piece {
                    piece_type: PieceTypes::Rook,
                    white: false,
                };
            }
        }

        self.board[m.to as usize] = temp;
        self.board[m.from as usize] = Piece {
            piece_type: PieceTypes::Empty,
            white: false,
        };

        if !self.wtomove {
            self.fullmoves += 1;
        }

        self.wtomove = !self.wtomove;

        move |board: &mut Board| {
            board.board = uboard;
            board.wtomove = uwtomove;
            board.white_ks = uwhite_ks;
            board.white_qs = uwhite_qs;
            board.black_ks = ublack_ks;
            board.black_qs = ublack_qs;
            board.enpassant_square = uenpassant_square;
            board.fullmoves = ufullmoves;
            board.precomputed_move_data = uprecomputed_move_data;
            board.bkingpos = ubkingpos;
            board.wkingpos = uwkingpos;
        }
    }
}

pub fn parse_square(square: &str) -> u8 {
    let rank_file = square.chars();
    let mut asquare: Vec<char> = Vec::new();

    for token in rank_file {
        asquare.push(token);
    }

    let rank: u8 = asquare[1].to_digit(10).unwrap() as u8;
    let filechr: char = asquare[0];

    let mut file: u8 = 0;

    if filechr == 'a' {
        file = 0;
    } else if filechr == 'b' {
        file = 1;
    } else if filechr == 'c' {
        file = 2;
    } else if filechr == 'd' {
        file = 3;
    } else if filechr == 'e' {
        file = 4;
    } else if filechr == 'f' {
        file = 5;
    } else if filechr == 'g' {
        file = 6;
    } else if filechr == 'h' {
        file = 7;
    }

    8 * (8 - rank) + file
}
