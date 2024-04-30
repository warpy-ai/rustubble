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
- [Progress bar Component](#progress-bar-component)

# TextInput Component

![textInput](https://github.com/warpy-ai/rustubble/blob/main/assets/input.png)

The `TextInput` component is a versatile and customizable input field designed for terminal-based applications using Rust. It supports Unicode, dynamic input handling, and comes with a variety of customization options to enhance user interaction.

## Features

- **Unicode Support**: Handles Unicode input seamlessly.
- **Dynamic Input Handling**: Supports real-time input processing including pasting and deletion.
- **Customization**: Allows setting up padding, placeholder, initial text, and helper text.
- **Cursor Management**: Manages cursor positioning and ensures it is always placed correctly based on user interaction.

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/text_input_example.rs)

# TextArea Component

![textArea](https://github.com/warpy-ai/rustubble/blob/main/assets/textarea.gif)

A text area field, akin to an <textarea /> in HTML. Allows for input that spans multiple lines. Supports unicode, pasting, vertical scrolling when the value exceeds the width and height of the element, and many customization options.

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/text_area_example.rs)

# Spinner Component

![spinner](https://github.com/warpy-ai/rustubble/blob/main/assets/spinner.gif)

The Spinner Component provides an easy-to-use and customizable loading animation for CLI applications written in Rust, utilizing the Crossterm library to handle terminal output. This component allows for a dynamic visual display during long-running operations.

## Features

- **Multiple Spinner Styles**: Choose from a variety of predefined spinner styles including dots, lines, and more complex patterns.
- **Customizable Speed**: Control the speed of the spinner animation.
- **Customizable Messages**: Attach messages alongside the spinner to provide real-time feedback to users.
- **Easy Integration**: Simple API for starting, updating, and stopping the spinner.

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/spinner_example.rs)

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

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/table_example.rs)

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

# Progress bar Component

The `ProgressBar` component is a versatile and customizable loading animation for CLI applications written in Rust. This component allows for a dynamic visual display during long-running operations.

![progress](https://github.com/warpy-ai/rustubble/blob/main/assets/progress_bar.gif)

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/progress_bar_example.rs)

# ViewPort Component

The `ViewPort` component allows you to create a viewport in your terminal application. The component provides a flexible and customizable way to display data in a `String` view format

![viewport](https://github.com/warpy-ai/rustubble/blob/main/assets/viewport.gif)

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/viewport_example.rs)

# StopWatch

A simple component for counting down.

![stopwatch](https://github.com/warpy-ai/rustubble/blob/main/assets/stopwatch.gif)

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/stopwatch_example.rs)

# Timer

A simple component for counting up.

![timer](https://github.com/warpy-ai/rustubble/blob/main/assets/timer.gif)

## Usage

- [Example Code](https://github.com/warpy-ai/rustubble/blob/main/examples/timer_example.rs)

## Contribution

Contributions are welcome! If you have suggestions for improving the spinner or adding new styles, please open an issue or pull request on our GitHub repository.

## License

This project is licensed under the Apache License - see the [LICENSE](https://github.com/warpy-ai/rustubble/blob/main/LICENSE.md) file for details.
