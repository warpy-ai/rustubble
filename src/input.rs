use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::helper::Helper;

pub struct TextInput {
    text: String,
    cursor_position: usize,
    placeholder: Option<String>,
    padding: usize,
    label: String,
    helper: Option<Helper>,
    prefix: String,
}

impl TextInput {
    pub fn new(
        placeholder: Option<&str>,
        padding: usize,
        initial_text: &str,
        label: &str,
        helper_text: Option<&str>,
        prefix: &str,
    ) -> Self {
        TextInput {
            text: initial_text.to_string(),
            cursor_position: initial_text.len(),
            placeholder: placeholder.map(String::from),
            padding,
            label: label.to_string(),
            helper: helper_text.map(|text| Helper::new(text)), // Initialize helper if provided
            prefix: prefix.to_string(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.text == self.placeholder.as_ref().map_or("", String::as_str) || self.text.is_empty()
        {
            self.text.clear(); // Clear the initial or placeholder text
            self.cursor_position = 0; // Reset the cursor position
        }
        self.text.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn render(&self, x: u16, y: u16) {
        // Move to the position and clear the line for the label
        execute!(
            std::io::stdout(),
            MoveTo(x + self.padding as u16, y),
            Clear(ClearType::CurrentLine),
            Print(&self.label),
        )
        .unwrap();

        execute!(
            std::io::stdout(),
            MoveTo(x, y + 2),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        execute!(
            std::io::stdout(),
            MoveTo(x, y + 2),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::White),
            Print(" ".repeat(self.padding)), // Left padding
            Print(format!(
                "{} ",
                if self.prefix.is_empty() {
                    ""
                } else {
                    self.prefix.as_str()
                }
            )), // Render the prefix
            SetForegroundColor(Color::Grey),
            Print(if self.text.is_empty() {
                self.placeholder.as_deref().unwrap_or("")
            } else {
                &self.text
            })
        )
        .unwrap();

        if let Some(ref helper) = self.helper {
            helper.render(x + self.padding as u16, y + 5);
            // Render helper text below the input field
        }

        // Reset cursor position and color
        execute!(
            std::io::stdout(),
            MoveTo(
                x + self.padding as u16 + self.prefix.len() as u16 + self.cursor_position as u16,
                y + 2
            ),
            SetForegroundColor(Color::Reset)
        )
        .unwrap();
        // Reset cursor position to after the text input
        execute!(
            std::io::stdout(),
            MoveTo(
                x + self.padding as u16 + self.prefix.len() as u16 + self.cursor_position as u16,
                y + 2
            )
        )
        .unwrap();
        // Reset the color to default
        execute!(std::io::stdout(), SetForegroundColor(Color::Reset)).unwrap();
    }
}

pub fn handle_input(input: &mut TextInput, x: u16, y: u16) {
    enable_raw_mode().unwrap();
    input.render(x, y);
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            }) => {
                if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                    break; // Break the loop if Ctrl+C is pressed
                }
                input.insert_char(c);
                input.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                input.delete_char();
                input.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                input.move_cursor_left();
                input.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                input.move_cursor_right();
                input.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            _ => {}
        }
    }
    disable_raw_mode().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn test_insert_char() {
        let mut text_input = TextInput::new(None, 0, "", "Label", None, "");
        text_input.insert_char('a');
        assert_eq!(text_input.text, "a");
        assert_eq!(text_input.cursor_position, 1);
    }

    #[test]
    fn test_delete_char() {
        let mut text_input = TextInput::new(None, 0, "a", "Label", None, "");
        text_input.delete_char();
        assert_eq!(text_input.text, "");
        assert_eq!(text_input.cursor_position, 0);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut text_input = TextInput::new(None, 0, "ab", "Label", None, "");
        text_input.move_cursor_right(); // Move cursor to end
        text_input.move_cursor_left();
        assert_eq!(text_input.cursor_position, 1);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut text_input = TextInput::new(None, 0, "abc", "Label", None, "");
        text_input.move_cursor_right();
        text_input.move_cursor_right();
        text_input.move_cursor_left();
        text_input.move_cursor_right(); // Should be at the end now
        assert_eq!(text_input.cursor_position, 3);
    }
}
