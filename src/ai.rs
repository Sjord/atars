use crate::game::{Board, Piece, Move};

pub struct ComputerPlayer {
}

impl ComputerPlayer {
    pub fn new() -> ComputerPlayer {
        ComputerPlayer {}
    }

    pub fn get_move(&self, board: &Board, turn: Piece) -> Move {
        let moves = board.get_moves(turn);
        moves[0]
    }
}