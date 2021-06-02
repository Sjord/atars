use coffee::graphics::{Color, Frame, Window, WindowSettings};
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
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
    }
}