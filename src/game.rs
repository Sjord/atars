use std::cmp::Ordering;
use std::ops::Index;
use std::ops::IndexMut;
use std::vec::Vec;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    White,
    Black,
}

impl Piece {
    fn other(&self) -> Piece {
        match self {
            Piece::White => Piece::Black,
            Piece::Black => Piece::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SquarePosition {
    pub x: usize,
    pub y: usize,
}

impl SquarePosition {
    pub fn new(x: usize, y: usize) -> SquarePosition {
        SquarePosition { x, y }
    }

    pub fn distance(&self, other: &SquarePosition) -> usize {
        let horizontal = abs_difference(self.x, other.x);
        let vertical = abs_difference(self.y, other.y);
        std::cmp::max(horizontal, vertical)
    }
}

pub struct Board {
    board : [Option<Piece>; 49]
}

impl Board {
    fn new() -> Board {
        let mut b = Board { board: [None; 49] };
        b[SquarePosition::new(0, 0)] = Some(Piece::Black);
        b[SquarePosition::new(6, 6)] = Some(Piece::Black);
        b[SquarePosition::new(0, 6)] = Some(Piece::White);
        b[SquarePosition::new(6, 0)] = Some(Piece::White);
        b
    }

    fn perform_move(&mut self, move_: &Move) -> bool {
        let p = self[move_.from];
        match p {
            None => false,
            Some(p) => {
                self[move_.to] = Some(p);
                self.turn_surrounding(p, move_.to);
                if move_.distance() == 2 {
                    self[move_.from] = None;
                }
                true
            }
        }
    }

    fn turn_surrounding(&mut self, mover: Piece, pos: SquarePosition) {
        for pos in self.get_surrounding(pos) {
            if self[pos] == Some(mover.other()) {
                self[pos] = Some(mover);
            }
        }
    }

    fn get_surrounding(&self, origin: SquarePosition) -> Vec<SquarePosition> {
        let mut squares = Vec::new();
        for pos in self.all_positions() {
            if origin.distance(&pos) == 1 {
                squares.push(pos);
            }
        }
        squares
    }

    fn all_positions(&self) -> Vec<SquarePosition>  {
        let mut squares = Vec::new();
        for x in 0..7 {
            for y in 0..7 {
                let pos = SquarePosition::new(x, y);
                squares.push(pos);
            }
        }
        squares
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|pos| *pos != None)
    }

    fn majority(&self) -> Option<Piece> {
        match self.count(Piece::White).cmp(&self.count(Piece::Black)) {
            Ordering::Greater => Some(Piece::White),
            Ordering::Equal => None,
            Ordering::Less => Some(Piece::Black),
        }
    }

    fn count(&self, piece: Piece) -> usize {
        self.board.iter().filter(|p| **p == Some(piece)).count()
    }

    pub fn pieces(&self) -> Vec<PiecePosition> {
        let mut result = Vec::new();
        for pos in self.all_positions() {
            if let Some(piece) = self[pos] {
                result.push(PiecePosition { pos, piece })
            }
        }
        result
    }

    fn blank_positions(&self) -> Vec<SquarePosition> {
        self.all_positions().into_iter().filter(|pos| self[*pos] == None).collect()
    }

    fn get_moves(&self, piece: Piece) -> Vec<Move> {
        let mut result = Vec::new();
        for to in self.blank_positions() {
            for from in self.get_surrounding(to) {
                if self[from] == Some(piece) {
                    result.push(Move::new(from, to));
                    break;
                }
            }
        }
        result
    }
}

pub struct PiecePosition {
    pub pos: SquarePosition,
    pub piece: Piece,
}

#[test]
fn test_get_surrounding() {
    let b = Board::new();
    let s = b.get_surrounding(SquarePosition::new(0, 0));
    assert_eq!(s.len(), 3);

    let s = b.get_surrounding(SquarePosition::new(3, 3));
    assert_eq!(s.len(), 8);

    let s = b.get_surrounding(SquarePosition::new(6, 3));
    assert_eq!(s.len(), 5);
}

#[test]
fn test_majority() {
    let mut b = Board::new();
    assert_eq!(b.majority(), None);

    b[SquarePosition::new(1, 1)] = Some(Piece::Black);
    assert_eq!(b.majority(), Some(Piece::Black));
    
    b[SquarePosition::new(1, 1)] = Some(Piece::White);
    assert_eq!(b.majority(), Some(Piece::White));
}

impl Index<SquarePosition> for Board {
    type Output = Option<Piece>;

    fn index(&self, pos: SquarePosition) -> &Option<Piece> {
        let offset = pos.x + pos.y * 7;
        &self.board[offset]
    }
}

impl IndexMut<SquarePosition> for Board {
    fn index_mut(&mut self, pos: SquarePosition) -> &mut Self::Output {
        let offset = pos.x + pos.y * 7;
        &mut self.board[offset]
    }
}

pub struct Atars {
    pub board : Board,
    pub turn: Piece,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    from: SquarePosition,
    to: SquarePosition
}

fn abs_difference<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl Move {
    pub fn new(from: SquarePosition, to: SquarePosition) -> Move {
        Move { from, to }
    }

    fn distance(&self) -> usize {
        self.from.distance(&self.to)
    }
}

#[test]
fn test_move_distance() {
    let from = SquarePosition::new(0, 0);
    let to = SquarePosition::new(1, 2);
    assert_eq!(Move::new(from, to).distance(), 2);
    assert_eq!(Move::new(from, to).distance(), Move::new(to, from).distance());
}

impl Atars {
    pub fn new() -> Atars {
        Atars { 
            board: Board::new(), 
            turn: Piece::White,
        }
    }

    fn is_valid_move(&self, move_: &Move) -> bool {
        move_.distance() <= 2 
        && self.board[move_.from] == Some(self.turn)
        && self.board[move_.to] == None
    }

    pub fn perform_move(&mut self, move_: Move) -> bool {
        let done_move = self.is_valid_move(&move_) && self.board.perform_move(&move_);
        if done_move {
            self.turn = self.turn.other();
        }
        done_move
    }
}