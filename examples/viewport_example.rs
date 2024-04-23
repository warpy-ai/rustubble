extern crate rustubble;

use std::fs;

use crossterm::terminal;
use rustubble::viewport::handle_viewport;
use rustubble::viewport::Viewport;
fn main() {
    terminal::enable_raw_mode().unwrap();
 

    let file_path = "examples/poem.md";

    // tod: get the file name from file_path

    let file_name = file_path.split('/').last().unwrap();
    let header = file_name.to_string();

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");



    let height = 20; // Assume the visible height of the content area is 5 lines
    let width = 100; // Assume the visible width of the content area is 10 characters

    let x = 5;
    let y = 5;

    let mut viewport = Viewport::new(header, content, height, width , 6);

    handle_viewport(&mut viewport, x, y);
    terminal::disable_raw_mode().unwrap();
    viewport.show_cursor();
}
