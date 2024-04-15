use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
};

pub struct Helper {
    text: String,
    color: Color,
}

impl Helper {
    pub fn new(text: &str) -> Self {
        Helper {
            text: text.to_string(),
            color: Color::DarkGrey, // Default color for helper text
        }
    }

    pub fn render(&self, x: u16, y: u16) {
        // Use crossterm to set color and position before printing the helper text
        execute!(
            std::io::stdout(),
            MoveTo(x, y),
            SetForegroundColor(self.color),
            Print(&self.text),
            SetForegroundColor(Color::Reset) // Reset color after rendering
        )
        .unwrap();
    }
}
