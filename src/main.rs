use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use coffee::input::Mouse;
use coffee::input::mouse::Button;
use nalgebra::geometry::Point2;

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

type SquarePosition = Point2<usize>;

struct Board {
    board : [Option<Piece>; 49]
}

impl Board {
    fn new() -> Board {
        let mut b = Board { board: [None; 49] };
        b.set(SquarePosition::new(0, 0), Some(Piece::Black));
        b.set(SquarePosition::new(6, 6), Some(Piece::Black));
        b.set(SquarePosition::new(0, 6), Some(Piece::White));
        b.set(SquarePosition::new(6, 0), Some(Piece::White));
        b
    }

    fn set(&mut self, pos: SquarePosition, new_value: Option<Piece>) {
        let offset = pos.x + pos.y * 7;
        self.board[offset] = new_value;
    }

    fn get(&self, pos: SquarePosition) -> Option<Piece> {
        let offset = pos.x + pos.y * 7;
        self.board[offset]
    }
}

struct Atars {
    board : Board,
    selected : Option<SquarePosition>
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
        let horizontal = abs_difference(self.from.x, self.to.x);
        let vertical = abs_difference(self.from.y, self.to.y);
        std::cmp::max(horizontal, vertical)
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
            selected: None 
        }
    }

    fn create_pieces_mesh(&self, frame: &Frame) -> Mesh {
        let mut mesh = Mesh::new();
        let grid = CoordinateGrid::new(frame.width(), frame.height());

        for x in 0..7 {
            for y in 0..7 {
                let pos = SquarePosition::new(x, y);
                let p = self.board.get(pos);
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
            None => self.selected = Some(pos),
            Some(old_pos) => self.perform_move(Move::new(old_pos, pos))
        }
    }

    fn perform_move(&self, move_: Move) {
        println!("{:?}", move_);
    }
}