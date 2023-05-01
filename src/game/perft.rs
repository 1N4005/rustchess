use super::{board::Board, movegen::generate_legal_moves};

pub fn go(depth: u8, board: &mut Board, depth_from_root: u8) -> u64 {
    if depth == 1 {
        let legal_moves = generate_legal_moves(board);

        // for m in &legal_moves {
        //     println!("{}: 1", m.uci());
        // }

        return legal_moves.len() as u64;
    }

    let mut count: u64 = 0;

    for m in generate_legal_moves(board) {
        let undo = board.push(&m);
        let numpositions = go(depth - 1, board, depth_from_root + 1);
        count += numpositions;
        undo(board);

        // if depth_from_root == 0 {
        //     println!("{}: {}", m.uci() ,numpositions)
        // }
    }

    count
}