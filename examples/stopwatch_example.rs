extern crate rustubble;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use rustubble::stopwatch::{handle_stopwatch, StopWatch};
use std::io;

fn main() {
    execute!(io::stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    let mut time = StopWatch::new();

    let (x, y) = (5, 5);
    handle_stopwatch(&mut time, x, y);

    execute!(io::stdout(), LeaveAlternateScreen).unwrap();

    disable_raw_mode().unwrap();
}
