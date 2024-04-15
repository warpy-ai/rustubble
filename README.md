# rustubble

Beautifull components for your terminal.

# Motive

This project aims to provide a set of components that can be used in your terminal applications.

# TextInput Component

The `TextInput` component is a versatile and customizable input field designed for terminal-based applications using Rust. It supports Unicode, dynamic input handling, and comes with a variety of customization options to enhance user interaction.

## Features

- **Unicode Support**: Handles Unicode input seamlessly.
- **Dynamic Input Handling**: Supports real-time input processing including pasting and deletion.
- **Customization**: Allows setting up padding, placeholder, initial text, and helper text.
- **Cursor Management**: Manages cursor positioning and ensures it is always placed correctly based on user interaction.

## Usage

```rust
use your_crate::TextInput; // Adjust the import path based on your lib structure

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
