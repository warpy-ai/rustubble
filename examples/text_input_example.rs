extern crate rustubble;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use rustubble::TextInput; // Adjust the import path based on your lib structure

fn main() -> std::io::Result<()> {
    enable_raw_mode()?; // Enable raw mode for direct terminal manipulation
    let text_input = TextInput::new(
        Some("Type here..."),   // Placeholder
        2,                      // Padding
        "Hello, World!",        // Initial text
        "Enter text:",          // Label
        Some("Ctrl+C to exit"), // Helper text
    );

    let x = 5; // x position in the terminal
    let y = 5; // y position in the terminal

    text_input.render(x, y);

    disable_raw_mode()?;
    // Add interaction handling here based on your application logic
    Ok(())
}
