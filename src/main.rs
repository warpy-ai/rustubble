mod colors;
mod spinner;
mod text_area;

use std::io::stdout;

use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use spinner::handle_spinner;
use spinner::Spinner;
fn main() {
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let spinner = Spinner::new(
        Color::Rgb {
            r: 0,
            g: 255,
            b: 255,
        },
        "Loading... Please wait.".to_string(),
    );

    let (x, y) = (10, 10);
    handle_spinner(&spinner, x, y);

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    println!("Operation completed.");
}
