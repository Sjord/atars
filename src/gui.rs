use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window, Rectangle};
use coffee::load::Task;
use coffee::{Game, Timer};
use coffee::input::Mouse;
use coffee::input::mouse::Button;

use crate::game::{SquarePosition, Move};

pub struct Atars {
    game: crate::game::Atars,
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

impl Into<Color> for crate::game::Piece {
    fn into(self) -> Color {
        match self {
            crate::game::Piece::White => Color::WHITE,
            crate::game::Piece::Black => Color::BLACK,
        }
    }
}

impl Atars {
    fn new() -> Atars {
        Atars { 
            game: crate::game::Atars::new(),
            selected: None,
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

        for pp in self.game.board.pieces() {
            let piece = Shape::Ellipse {
                center: grid.center(pp.pos),
                horizontal_radius: grid.piece_radius(),
                vertical_radius: grid.piece_radius(),
                rotation: 0.0
            };
            mesh.fill(piece, pp.piece.into());
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
                if self.game.board[pos] == Some(self.game.turn) {
                    self.selected = Some(pos);
                }
            }
            Some(old_pos) => {
                self.game.perform_move(Move::new(old_pos, pos));
                self.selected = None;
            }
        }
    }
}