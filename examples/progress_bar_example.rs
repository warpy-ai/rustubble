extern crate rustubble;
use crossterm::cursor::Show;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, ExecutableCommand};
use rustubble::colors::custom::{DARK_WHITE, ORANGE};
use rustubble::progress_bar::handle_progress_bar;
use rustubble::progress_bar::ProgressBar;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Example usage
    let mut stdout = stdout();

    let progress = 1.0; // 50% progress
    let length = 50; // Total length of the progress bar
    let start_color = DARK_WHITE; // Red
    let end_color = ORANGE; // Green

    let mut progress_bar = ProgressBar::new(
        "Dowloading...".to_string(),
        progress,
        length,
        start_color,
        end_color,
    );
    let (x, y) = (10, 10);

    stdout.execute(Clear(ClearType::All)).unwrap();

    for i in 0..=100 {
        handle_progress_bar(&mut progress_bar, i as f32 / 100.0, x, y);

        // progress_bar.update(i);
        sleep(Duration::from_millis(10)); // Simulate time-consuming task
    }

    // Show the cursor after loading finished
    execute!(stdout, Show).unwrap();
}
