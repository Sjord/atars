use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window, WindowSettings, Rectangle};
use coffee::load::Task;
use coffee::{Game, Timer};
use coffee::input::Mouse;
use coffee::input::mouse::Button;
use nalgebra::geometry::Point2;
use std::cmp::Ordering;
use std::ops::Index;
use std::ops::IndexMut;
use std::vec::Vec;


fn main() {
    Atars::run(WindowSettings {
        title: String::from("Atars"),
        size: (1024, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    }).unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Piece {
    White,
    Black,
}

impl Into<Color> for Piece {
    fn into(self) -> Color {
        match self {
            Piece::White => Color::WHITE,
            Piece::Black => Color::BLACK,
        }
    }
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
struct SquarePosition {
    x: usize,
    y: usize,
}

impl SquarePosition {
    fn new(x: usize, y: usize) -> SquarePosition {
        SquarePosition { x, y }
    }

    fn distance(&self, other: &SquarePosition) -> usize {
        let horizontal = abs_difference(self.x, other.x);
        let vertical = abs_difference(self.y, other.y);
        std::cmp::max(horizontal, vertical)
    }
}

struct Board {
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

struct Atars {
    board : Board,
    selected : Option<SquarePosition>,
    turn: Piece,
}

impl Game for Atars {
    type Input = Mouse;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Atars> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| Atars::new())
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLUE);

        let pieces = self.create_pieces_mesh(frame);
        pieces.draw(&mut frame.as_target());

        let grid = self.create_grid_mesh(frame);
        grid.draw(&mut frame.as_target());

        let selected = self.create_selected_mesh(frame);
        selected.draw(&mut frame.as_target());
    }

    fn interact(&mut self, input: &mut Mouse, window: &mut Window) {
        let clicks = input.button_clicks(Button::Left);

        let grid = CoordinateGrid::new(window.width(), window.height());
        for point in clicks {
            let pos = grid.from_point(point);
            self.handle_click(pos);
        }
    }
}

struct CoordinateGrid {
    width: f32,
    height: f32,
}

impl CoordinateGrid {
    fn new(width: f32, height: f32) -> CoordinateGrid {
        CoordinateGrid { width, height }
    }

    /// get the point in the center of the square
    fn center(&self, pos: SquarePosition) -> Point {
        Point::new(
            (0.5 + pos.x as f32) * (self.width / 7.),
            (0.5 + pos.y as f32) * (self.height / 7.)
        )
    }

    fn rectangle(&self, pos: SquarePosition) -> Rectangle<f32> {
        Rectangle {
            x: (pos.x as f32) * (self.width / 7.),
            y: (pos.y as f32) * (self.height / 7.),
            width: self.width / 7.,
            height: self.height / 7.
        }
    }

    fn piece_radius(&self) -> f32 {
        self.width / 20.
    }

    fn from_point(&self, point: &Point) -> SquarePosition {
        SquarePosition::new(
            (point.x / self.width * 7.) as usize,
            (point.y / self.height * 7.) as usize
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Move {
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
    fn new(from: SquarePosition, to: SquarePosition) -> Move {
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
    fn new() -> Atars {
        Atars { 
            board: Board::new(), 
            selected: None,
            turn: Piece::White,
        }
    }

    fn create_selected_mesh(&self, frame: &Frame) -> Mesh {
        let grid = CoordinateGrid::new(frame.width(), frame.height());
        let mut mesh = Mesh::new();

        if let Some(sel_pos) = self.selected {
            let rect = Shape::Rectangle(grid.rectangle(sel_pos));
            mesh.stroke(rect, Color::RED, 1.0)
        }

        mesh
    }

    fn create_pieces_mesh(&self, frame: &Frame) -> Mesh {
        let mut mesh = Mesh::new();
        let grid = CoordinateGrid::new(frame.width(), frame.height());

        for x in 0..7 {
            for y in 0..7 {
                let pos = SquarePosition::new(x, y);
                let p = self.board[pos];
                if let Some(p) = p {
                    let piece = Shape::Ellipse {
                        center: grid.center(pos),
                        horizontal_radius: grid.piece_radius(),
                        vertical_radius: grid.piece_radius(),
                        rotation: 0.0
                    };
                    mesh.fill(piece, p.into());
                }
            }
        }

        mesh
    }

    fn create_grid_mesh(&self, frame: &Frame) -> Mesh {
        let mut mesh = Mesh::new();
        let space = frame.width() / 7.;
        for x in 1..7 {
            let x = x as f32 * space;
            let line = Shape::Polyline {
                points: vec!(Point::new(x, 0.), Point::new(x, frame.height()))
            };
            mesh.stroke(line, Color::BLACK, 1.0);
        }

        let space = frame.height() / 7.;
        for y in 1..7 {
            let y = y as f32 * space;
            let line = Shape::Polyline {
                points: vec!(Point::new(0., y), Point::new(frame.width(), y))
            };
            mesh.stroke(line, Color::BLACK, 1.0);
        }
        mesh
    }

    fn handle_click(&mut self, pos: SquarePosition) {
        match self.selected {
            None => {
                if self.board[pos] == Some(self.turn) {
                    self.selected = Some(pos);
                }
            }
            Some(old_pos) => self.perform_move(Move::new(old_pos, pos))
        }
    }

    fn is_valid_move(&self, move_: &Move) -> bool {
        move_.distance() <= 2 
        && self.board[move_.from] == Some(self.turn)
        && self.board[move_.to] == None
    }

    fn perform_move(&mut self, move_: Move) {
        if self.is_valid_move(&move_) && self.board.perform_move(&move_) {
            self.turn = self.turn.other();
        }
        self.selected = None;
    }
}