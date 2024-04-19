# rustubble

Beautifull components for your terminal.

[![Rust](https://github.com/warpy-ai/rustubble/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/warpy-ai/rustubble/actions/workflows/rust.yml)

# Motive

This project aims to provide a set of components that can be used in your terminal applications.

# Components

- [TextInput Component](#textinput-component)
- [TextArea Component](#textarea-component)
- [Spinner Component](#spinner-component)
- [Table Component](#table-component)

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

# Spinner Component

The Spinner Component provides an easy-to-use and customizable loading animation for CLI applications written in Rust, utilizing the Crossterm library to handle terminal output. This component allows for a dynamic visual display during long-running operations.

![spinner](https://github.com/warpy-ai/rustubble/blob/main/assets/spinner.gif)

## Features

- **Multiple Spinner Styles**: Choose from a variety of predefined spinner styles including dots, lines, and more complex patterns.
- **Customizable Speed**: Control the speed of the spinner animation.
- **Customizable Messages**: Attach messages alongside the spinner to provide real-time feedback to users.
- **Easy Integration**: Simple API for starting, updating, and stopping the spinner.

## Usage

```rust
use std::io::stdout;

use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rustubble::spinner::handle_spinner;
use rustubble::spinner::Spinner;

fn main() {
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let spinner = Spinner::new(
        Color::Rgb {
            r: 0,
            g: 255,
            b: 255,
        },
        "Loading... Please wait.".to_string(),
        "FingerDance",
    );

    let (x, y) = (10, 10);
    handle_spinner(&spinner, x, y);

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    println!("Operation completed.");
}
```

### Customizing the Spinner

You can customize the spinner style and message at initialization. Here's how you can specify a different spinner style:

```rust
let spinner = Spinner::new("Dots2", "Loading resources...");
```

Available styles include `Dots`, `Dots2`, `Dots3`, etc. Refer to the `spinner_data.rs` file for a complete list of available styles and their configurations.

## Spinner Styles

The spinner styles are predefined in a `lazy_static` block within the `spinner_data.rs` file. Each style is represented by a unique key and includes an array of frames and an interval timing in milliseconds.

Here’s an excerpt from the spinner styles definition:

```rust
lazy_static! {
    static ref SPINNERS: HashMap<String, SpinnerData> = {
        hashmap! {
            "Dots".into() => SpinnerData {
                frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                interval: 80
            },
            // Additional spinner styles...
        }
    };
}
```

# Table Component

The `Table` component allows you to create a table in your terminal application using Rust. The component provides a flexible and customizable way to display data in a table format.

![table](https://github.com/warpy-ai/rustubble/blob/main/assets/table.png)

## Usage

```rust
fn main() -> std::io::Result<()> {
    enable_raw_mode()?; // Enable raw mode for direct terminal manipulation

    let headers = vec![
        "Rank".to_string(),
        "City".to_string(),
        "Country".to_string(),
        "Population".to_string(),
    ];

    let data = vec![
        vec![
            "1".to_string(),
            "Tokyo".to_string(),
            "Japan".to_string(),
            "37,274,000".to_string(),
        ],
        vec![
            "2".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "3".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "4".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "5".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "6".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "7".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        // Add more rows as necessary
    ];

    let mut table = Table::new(headers, data, 0, 3, 7); // Selected row is 0, padding is 1

    let (x, y) = (5, 5);
    handle_table(&mut table, x, y);

    // Clean up the terminal
    disable_raw_mode()?;
    // Clean up before exiting

    Ok(())
}
```

### Customizing the Table

You can set the padding, the number of visible lines and the scroll offset of the table. Here's an example of how you can customize the table:

```rust
let mut table = Table::new(headers, data, 0, 3, 7);
```

You can set the position of the table on the view when rendering:

```rust
 let (x, y) = (5, 5);
  handle_table(&mut table, x, y);
```

## Contribution

Contributions are welcome! If you have suggestions for improving the spinner or adding new styles, please open an issue or pull request on our GitHub repository.

## License

This project is licensed under the Apache License - see the [LICENSE](https://github.com/warpy-ai/rustubble/blob/main/LICENSE.md) file for details.
