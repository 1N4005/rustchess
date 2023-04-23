pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Copy, Clone)]
pub enum PieceTypes {
    Empty,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

pub struct Board {
    pub board: [Piece; 64],
    pub white: Vec<u8>,
    pub black: Vec<u8>,

    pub wtomove: bool,

    pub white_ks: bool,
    pub white_qs: bool,
    pub black_ks: bool,
    pub black_qs: bool,
    pub enpassant_square: u8,
    pub fullmoves: u16
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub white: bool
}

impl Board {
    pub fn draw(&self) {
        let mut i: u8 = 0;
        for piece in self.board {
            match piece.piece_type {
                PieceTypes::Empty => print!("-"),
                PieceTypes::Pawn => print!("{}", if piece.white {"P"} else {"p"}),
                PieceTypes::Bishop => print!("{}", if piece.white {"B"} else {"b"}),
                PieceTypes::Knight => print!("{}", if piece.white {"N"} else {"n"}),
                PieceTypes::Rook => print!("{}", if piece.white {"R"} else {"r"}),
                PieceTypes::Queen => print!("{}", if piece.white {"Q"} else {"q"}),
                PieceTypes::King => print!("{}", if piece.white {"K"} else {"k"}),
            }

            if i % 8 == 7 {
                println!();
            }
            i += 1;
        }
    }


    pub fn new(fen: &str) -> Self {
        let fen_array = fen.split(" ");
        let mut tokens: [&str; 6] = [""; 6];

        let mut i = 0;

        for token in fen_array {
            tokens[i] = token;
            i += 1;
        }

        let board_array = tokens[0].split("/");

        let mut board: [Piece; 64] = [Piece{piece_type: PieceTypes::Empty, white: false}; 64];
        let mut white: Vec<u8> = Vec::new();
        let mut black: Vec<u8> = Vec::new();
        let mut white_ks: bool = false;
        let mut white_qs: bool = false;
        let mut black_ks: bool = false;
        let mut black_qs: bool = false;
        let mut enpassant_square: u8 = 0;
        let mut fullmoves: u16 = 1;
        let  wtomove: bool = tokens[1] == "w";

        let mut curr_square: usize = 0;

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
                        board[curr_square] = Piece{piece_type: PieceTypes::Pawn, white: false};
                        black.push(curr_square as u8);
                    } else if square == 'b' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Bishop, white: false};
                        black.push(curr_square as u8);
                    } else if square == 'n' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Knight, white: false};
                        black.push(curr_square as u8);
                    } else if square == 'r' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Rook, white: false};
                        black.push(curr_square as u8);
                    } else if square == 'q' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Queen, white: false};
                        black.push(curr_square as u8);
                    } else if square == 'k' {
                        board[curr_square] = Piece{piece_type: PieceTypes::King, white: false};
                        black.push(curr_square as u8);
                    } else if square == 'P' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Pawn, white: true};
                        white.push(curr_square as u8);
                    } else if square == 'B' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Bishop, white: true};
                        white.push(curr_square as u8);
                    } else if square == 'N' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Knight, white: true};
                        white.push(curr_square as u8);
                    } else if square == 'R' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Rook, white: true};
                        white.push(curr_square as u8);
                    } else if square == 'Q' {
                        board[curr_square] = Piece{piece_type: PieceTypes::Queen, white: true};
                        white.push(curr_square as u8);
                    } else if square == 'K' {
                        board[curr_square] = Piece{piece_type: PieceTypes::King, white: true};
                        white.push(curr_square as u8);
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

        if tokens[3] == "-" {
            enpassant_square = 64;
        } else {
            enpassant_square = parse_square(tokens[3]);
        }

        fullmoves = tokens[5].parse().unwrap();

        Board{
            board: board, 
            white: white, 
            black: black, 
            wtomove: wtomove,
            white_ks: white_ks, 
            white_qs: white_qs, 
            black_ks: black_ks, 
            black_qs: black_qs, 
            enpassant_square: enpassant_square, 
            fullmoves: fullmoves}
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
