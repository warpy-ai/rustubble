mod colors;
mod text_area;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use text_area::handle_text_area;
use text_area::TextArea;

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
