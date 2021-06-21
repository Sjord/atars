use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

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
enum Square {
    Empty,
    White,
    Black,
}

impl Into<Option<Color>> for Square {
    fn into(self) -> Option<Color> {
        match self {
            Square::White => Some(Color::WHITE),
            Square::Black => Some(Color::BLACK),
            Square::Empty => None
        }
    }
}

struct Board {
    board : [Square; 49]
}

impl Board {
    fn new() -> Board {
        let mut b = Board { board: [Square::Empty; 49] };
        b.set(0, 0, Square::Black);
        b.set(6, 6, Square::Black);
        b.set(0, 6, Square::White);
        b.set(6, 0, Square::White);
        b
    }

    fn set(&mut self, x: usize, y: usize, new_value: Square) {
        let offset = x + y * 7;
        self.board[offset] = new_value;
    }

    fn get(&self, x: usize, y: usize) -> Square {
        let offset = x + y * 7;
        self.board[offset]
    }
}

struct Atars {
    board : Board
}

impl Game for Atars {
    type Input = (); // No input data
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
}

impl Atars {
    fn new() -> Atars {
        Atars { board: Board::new() }
    }

    fn create_pieces_mesh(&self, frame: &Frame) -> Mesh {
        let mut mesh = Mesh::new();
        let space = frame.width() / 7.;

        for x in 0..7 {
            for y in 0..7 {
                let p = self.board.get(x, y);
                if p != Square::Empty {
                    let piece = Shape::Ellipse {
                        center: Point::new((0.5 + x as f32) * space, 
                        (0.5 + y as f32) * space),
                        horizontal_radius: space * 0.4,
                        vertical_radius: space * 0.4,
                        rotation: 0.0
                    };
                    let color : Option<Color> = p.into();
                    mesh.fill(piece, color.unwrap());
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
}