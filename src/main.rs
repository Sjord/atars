use coffee::graphics::WindowSettings;
use coffee::Game;

mod game;
mod gui;
mod ai;

fn main() {
    gui::Atars::run(WindowSettings {
        title: String::from("Atars"),
        size: (1024, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    }).unwrap();
}