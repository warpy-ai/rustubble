use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;

pub struct TextArea {
    text: Vec<String>, // Stores multiple lines of text
    cursor_x: usize,
    cursor_y: usize,
    scroll_offset: usize, // Top visible line index
    label: String,
    helper: Option<String>,
    visible_lines: usize,
}

impl TextArea {
    pub fn new(label: &str, helper: Option<&str>, visible_lines: usize) -> Self {
        TextArea {
            text: vec![String::new()], // Start with one empty line
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
            label: label.to_string(),
            helper: helper.map(String::from),
            visible_lines,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if c == '\n' {
            self.insert_new_line();
        } else {
            if self.cursor_x >= self.text[self.cursor_y].len() {
                self.text[self.cursor_y].push(c);
            } else {
                self.text[self.cursor_y].insert(self.cursor_x, c);
            }
            self.cursor_x += 1;
        }
        self.ensure_cursor_within_bounds();
    }

    pub fn insert_new_line(&mut self) {
        let current_line = self.text[self.cursor_y].split_off(self.cursor_x);
        self.text.insert(self.cursor_y + 1, current_line);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.adjust_scroll();
    }

    // Make sure cursor bounds are always respected
    pub fn ensure_cursor_within_bounds(&mut self) {
        if self.cursor_y >= self.text.len() {
            self.cursor_y = self.text.len() - 1;
        }
        if self.cursor_x > self.text[self.cursor_y].len() {
            self.cursor_x = self.text[self.cursor_y].len();
        }
    }

    // Adjust the scroll if cursor moves outside the visible area
    pub fn adjust_scroll(&mut self) {
        if self.cursor_y < self.scroll_offset {
            self.scroll_offset = self.cursor_y;
        } else if self.cursor_y >= self.scroll_offset + self.visible_lines {
            self.scroll_offset = self.cursor_y - self.visible_lines + 1;
        }
    }

    fn update_cursor_position(&self, x: u16, y: u16) {
        let current_line_y = y + 2 + (self.cursor_y - self.scroll_offset) as u16;
        let cursor_pos_x = x + 5 + self.cursor_x as u16; // Account for line number width
        let mut stdout = stdout();
        execute!(stdout, MoveTo(cursor_pos_x, current_line_y)).unwrap();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.text[self.cursor_y].len(); // Move to the end of the previous line
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_x < self.text[self.cursor_y].len() {
            self.cursor_x += 1;
        } else if self.cursor_y < self.text.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = 0; // Move to the start of the next line
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.text.len() - 1 {
            self.cursor_y += 1; // Move cursor down within the text boundaries

            // Adjust scroll offset if the cursor moves below the visible area
            if self.cursor_y >= self.scroll_offset + self.visible_lines {
                self.scroll_offset += 1;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            // Ensure that scroll_offset is adjusted if necessary
            if self.cursor_y < self.scroll_offset {
                self.scroll_offset -= 1;
            }
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.text[self.cursor_y].remove(self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            // Store the current line in a temporary variable before removing it
            let current_line = self.text.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.text[self.cursor_y].len();
            self.text[self.cursor_y].push_str(&current_line);
            if self.cursor_y < self.scroll_offset {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
        }
    }

    pub fn render(&self, x: u16, y: u16) {
        let mut stdout = stdout();

        // Extend clearing to ensure all lines including potential overflow are cleared
        let total_lines_to_clear = self.visible_lines + 3; // +3 to include label and two lines for helper just in case
        for i in 0..total_lines_to_clear {
            execute!(
                stdout,
                MoveTo(x, y + i as u16),
                Clear(ClearType::CurrentLine)
            )
            .unwrap();
        }

        // Render the label at the designated position
        execute!(stdout, MoveTo(x, y), Print(&self.label)).unwrap();

        // Render each line of text with its line number
        for i in 0..self.visible_lines {
            let line_idx = i + self.scroll_offset;
            execute!(
                stdout,
                MoveTo(x, y + 2 + i as u16), // +2 to offset from label
                Print(format!("|{:3} ", line_idx + 1)),
                Print(&self.text.get(line_idx).unwrap_or(&String::new()))
            )
            .unwrap();
        }

        // Render the helper text below the last visible line
        if let Some(helper_text) = &self.helper {
            execute!(
                stdout,
                MoveTo(x, y + 2 + self.visible_lines as u16 + 1),
                Print(helper_text)
            )
            .unwrap();
        }

        // Update the cursor position to reflect the latest changes
        self.update_cursor_position(x, y);
    }

    // Methods to handle input, scroll, etc., go here
}

pub fn handle_text_area(text_area: &mut TextArea, x: u16, y: u16) {
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
                text_area.insert_char(c);
                text_area.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                text_area.delete_char();
                text_area.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                text_area.move_cursor_left();
                text_area.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                text_area.move_cursor_right();
                text_area.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                text_area.move_cursor_down();
                text_area.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                text_area.move_cursor_up();
                text_area.render(x, y);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                text_area.insert_new_line();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,

            _ => {}
        }
        // Re-render text area after each input
        text_area.render(x, y);
    }
}
