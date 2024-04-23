extern crate rustubble;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use rustubble::text_area::handle_text_area;
use rustubble::text_area::TextArea;
use std::io::stdout;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut text_area = TextArea::new("Type here:", Some("Press ESC to exit."), 6);
    // text_area.render(0, 1); // Initial render at position (0, 1)

    let x = 5;
    let y = 5;
    let text_area_value = handle_text_area(&mut text_area, x, y);

    let text_2 = format!("Input value: {:?}", text_area_value);

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(
        std::io::stdout(),
        MoveTo(x, y),
        Clear(ClearType::CurrentLine),
        Print(text_2),
    )
    .unwrap();

    Ok(())
}
