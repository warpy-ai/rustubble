extern crate rustubble;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use rustubble::timer::{handle_timer, Timer};
use std::io;
use std::time::Duration;

fn main() {
    execute!(io::stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    let duration = Duration::new(5, 0); // For example, 5 minutes
    let mut timer = Timer::new(duration);

    let (x, y) = (5, 5);
    handle_timer(&mut timer, x, y);

    execute!(io::stdout(), LeaveAlternateScreen).unwrap();

    //sleep for 5 seconds
    disable_raw_mode().unwrap();
}
