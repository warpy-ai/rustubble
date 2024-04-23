extern crate rustubble;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use rustubble::{input::handle_input, input::TextInput}; // Adjust the import path based on your lib structure

fn main() {
    enable_raw_mode().unwrap();
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
    let input_value = handle_input(&mut text_input, x, y + 1);
    let text_2 = format!("Input value: {:?}", input_value);
    execute!(
        std::io::stdout(),
        MoveTo(x, y),
        Clear(ClearType::CurrentLine),
        Print(text_2),
    )
    .unwrap();

    disable_raw_mode().unwrap();
}
