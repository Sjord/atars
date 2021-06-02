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
struct Atars {
}

impl Game for Atars {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Atars> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| Atars { /* ... */ })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLUE);

        let piece = Shape::Ellipse {
            center: Point::new(100., 100.),
            horizontal_radius: 40.,
            vertical_radius: 40.,
            rotation: 0.0
        };
        let mut mesh = Mesh::new();
        mesh.fill(piece, Color::WHITE);

        mesh.draw(&mut frame.as_target());

        let grid = self.create_grid_mesh(frame);
        grid.draw(&mut frame.as_target());
    }
}

impl Atars {
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