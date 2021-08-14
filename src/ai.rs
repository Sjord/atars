use crate::game::{Board, Piece, Move};

pub struct ComputerPlayer {
}

impl ComputerPlayer {
    pub fn new() -> ComputerPlayer {
        ComputerPlayer {}
    }

    pub fn get_move(&self, board: &Board, turn: Piece) -> Move {
        let moves = board.get_moves(turn);
        moves.into_iter().max_by_key(|m| {
            let mut hypo_board=  board.clone();
            hypo_board.perform_move(m);
            hypo_board.count(turn)
        }).unwrap()
    }
}