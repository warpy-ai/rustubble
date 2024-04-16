# rustubble

Beautifull components for your terminal.

# Motive

This project aims to provide a set of components that can be used in your terminal applications.

# TextInput Component

The `TextInput` component is a versatile and customizable input field designed for terminal-based applications using Rust. It supports Unicode, dynamic input handling, and comes with a variety of customization options to enhance user interaction.

![textInput](https://github.com/warpy-ai/rustubble/blob/main/assets/input.png)

## Features

- **Unicode Support**: Handles Unicode input seamlessly.
- **Dynamic Input Handling**: Supports real-time input processing including pasting and deletion.
- **Customization**: Allows setting up padding, placeholder, initial text, and helper text.
- **Cursor Management**: Manages cursor positioning and ensures it is always placed correctly based on user interaction.

## Usage

```rust
use rustubble::TextInput; // Adjust the import path based on your lib structure

fn main() {
    let mut text_input = TextInput::new(
        Some("Type here..."), // Placeholder
        2,                    // Padding
        "Hello, World!",      // Initial text
        "Enter text:",        // Label
        Some("Ctrl+C to exit") // Helper text
    );

    let x = 5; // x position in the terminal
    let y = 5; // y position in the terminal

    text_input.render(x, y);
    // Add interaction handling here based on your application logic
}
```

# TextArea Component

A text area field, akin to an <textarea /> in HTML. Allows for input that spans multiple lines. Supports unicode, pasting, vertical scrolling when the value exceeds the width and height of the element, and many customization options.

![textArea](https://github.com/warpy-ai/rustubble/blob/main/assets/textarea.gif)

## Usage

```rust
use rustubble::handle_text_area;
use rustubble::TextArea;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut text_area = TextArea::new("Type here:", Some("Press ESC to exit."), 6);
    text_area.render(0, 1); // Initial render at position (0, 1)

    handle_text_area(&mut text_area, 0, 1);

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
```
