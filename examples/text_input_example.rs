extern crate rustubble;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

use rustubble::{input::handle_input, TextInput}; // Adjust the import path based on your lib structure

fn main() {
    let mut text_input = TextInput::new(
        Some("Type here..."),   // Placeholder
        2,                      // Padding
        "Hello",                // Initial text
        "Enter text:",          // Label
        Some("Ctrl+C to exit"), // Helper text
        ">",                    // Prefix
    );

    let x = 5;
    let y = 5;

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    // Assuming handle_input is defined to manage user interaction
    handle_input(&mut text_input, x, y + 1);
}
