mod input;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use input::handle_input;
use input::TextInput;

mod colors;
mod helper;

fn main() {
    let mut text_input = TextInput::new(
        Some("Type here..."),
        2,
        "",
        "Input: ",
        Some("Ctrl+C to exit, Enter to submit"),
    );

    // Setting the cursor position and terminal attributes
    let x = 5;
    let y = 5;

    // Clear the screen initially and render the input field
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(x, y)).unwrap();

    text_input.render(x, y);

    // Loop to handle input, breaking on specific conditions (like pressing Esc)
    handle_input(&mut text_input, x, y);

    text_input.render(x, y);
}
