use std::io::{stdout, Write};

use crate::colors::custom::PURPLE;
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

// execute!(stdout, Clear(ClearType::All)).unwrap();
pub struct Table {
    table_headers: Vec<String>,
    table_data: Vec<Vec<String>>,
    selected_row: usize,
    table_width: usize,
    padding: usize,
    visible_lines: usize,
    scroll_offset: usize,
}

impl Table {
    pub fn new(
        table_headers: Vec<String>,
        table_data: Vec<Vec<String>>,
        selected_row: usize,
        padding: usize,
        visible_lines: usize,
    ) -> Self {
        let widths: Vec<usize> =
            Self::calculate_column_widths(&table_headers, &table_data, padding);

        let table_width = widths.iter().sum::<usize>() + widths.len() + 1;

        Table {
            table_headers,
            table_data,
            selected_row,
            table_width,
            padding,
            visible_lines,
            scroll_offset: 0,
        }
    }

    fn calculate_column_widths(
        headers: &[String],
        data: &[Vec<String>],
        padding: usize,
    ) -> Vec<usize> {
        let mut widths = vec![0; headers.len()];

        // Calculate max width for each column based on headers and data
        for (i, header) in headers.iter().enumerate() {
            widths[i] = widths[i].max(header.len() + 2 * padding);
            for row in data {
                if row[i].len() + 2 * padding > widths[i] {
                    widths[i] = row[i].len() + 2 * padding;
                }
            }
        }
        widths
    }

    pub fn render(&self, x: u16, y: u16) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let column_widths =
            Self::calculate_column_widths(&self.table_headers, &self.table_data, self.padding);

        // Render the top border
        self.render_top_border(x, y, &column_widths);

        // Render table headers
        self.render_headers(x, y + 1);

        // Render separator line
        self.render_horizontal_line(x, y + 2, self.table_width, &column_widths);

        // Render table rows with total lines to clear
        let start_row = self.scroll_offset;
        let end_row = usize::min(
            self.scroll_offset + self.visible_lines,
            self.table_data.len(),
        );

        for (idx, row) in self.table_data[start_row..end_row].iter().enumerate() {
            let is_selected = (self.scroll_offset + idx) == self.selected_row;
            self.render_row(x, y + 3 + idx as u16, row, is_selected);
        }

        // Render the bottom border
        self.render_bottom_border(x, y + 3, &column_widths);

        //TODO: remove cursor
        self.hide_cursor();
        stdout.flush().unwrap();
    }

    fn render_row(&self, x: u16, y: u16, items: &[String], selected: bool) {
        let mut cursor_x = x;

        execute!(stdout(), MoveTo(cursor_x, y)).unwrap();
        print!("│");
        cursor_x += 1;
        let padding = " ".repeat(self.padding);

        let column_widths =
            Self::calculate_column_widths(&self.table_headers, &self.table_data, padding.len());

        for (idx, item) in items.iter().enumerate() {
            let cell_width = column_widths[idx]; // The calculated width for this column
            let padded_item = format!(
                "{}{:^cell_width$}{}",
                padding,
                item,
                padding,
                cell_width = cell_width - 2 * padding.len()
            ); // Right-align the text within the space
            execute!(stdout(), MoveTo(cursor_x, y)).unwrap();
            if selected {
                execute!(stdout(), SetBackgroundColor(PURPLE)).unwrap();
                print!("{padded_item}"); // Print item within background
                execute!(stdout(), ResetColor).unwrap();
            } else {
                print!("{padded_item}");
            }
            cursor_x += column_widths[idx as usize] as u16;
        }

        print!("│");
        println!();
    }

    fn render_headers(&self, x: u16, y: u16) {
        let mut stdout = stdout();
        let mut cursor_x = x;
        execute!(stdout, MoveTo(cursor_x, y)).unwrap();
        print!("│");
        cursor_x += 1;

        let column_widths =
            Self::calculate_column_widths(&self.table_headers, &self.table_data, self.padding);

        for (idx, header) in self.table_headers.iter().enumerate() {
            let padding = " ".repeat(self.padding);
            let content = format!("{}{}{}", padding, header, padding);
            execute!(stdout, MoveTo(cursor_x, y)).unwrap();
            execute!(stdout, Print(content), ResetColor).unwrap();
            // TOOD: calulate the header width with table_width
            cursor_x += column_widths[idx as usize] as u16;
        }
        print!("│");
        println!(); // Move to the next line after headers
    }

    fn render_horizontal_line(
        &self,
        x: u16,
        y: u16,
        total_width: usize,
        column_widths: &Vec<usize>,
    ) {
        let bottom_border = format!("│{}│", "─".repeat(total_width - (column_widths.len() + 1)));
        // let line = "─".repeat(total_width);
        execute!(stdout(), MoveTo(x, y), Print(bottom_border)).unwrap();
    }

    fn render_top_border(&self, x: u16, y: u16, column_widths: &Vec<usize>) {
        let mut stdout = stdout();
        let top_border = format!(
            "┌{}┐",
            "─".repeat(self.table_width - (column_widths.len() + 1))
        );
        execute!(stdout, MoveTo(x, y)).unwrap();
        println!("{}", top_border);
    }

    fn render_bottom_border(&self, x: u16, y: u16, column_widths: &Vec<usize>) {
        let mut stdout = stdout();
        let bottom_border = format!(
            "└{}┘",
            "─".repeat(self.table_width - (column_widths.len() + 1))
        );
        execute!(stdout, MoveTo(x, y + (self.visible_lines) as u16)).unwrap();
        println!("{}", bottom_border);
    }

    pub fn move_cursor_down(&mut self) {
        if self.selected_row < self.table_data.len() - 1 {
            self.selected_row += 1;
            if self.selected_row >= self.scroll_offset + self.visible_lines {
                self.scroll_offset += 1; // Scroll down
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            if self.selected_row < self.scroll_offset {
                self.scroll_offset -= 1; // Scroll up
            }
        }
    }

    fn hide_cursor(&self) {
        print!("\x1B[?25l"); // Hide the cursor
        std::io::stdout().flush().unwrap(); // Ensure the command is applied immediately
    }

    pub fn show_cursor(&self) {
        print!("\x1B[?25h"); // Show the cursor
        std::io::stdout().flush().unwrap(); // Ensure the command is applied immediately
    }
}

pub fn handle_table(table: &mut Table, x: u16, y: u16) {
    // Clear the screen initially to start with a clean slate
    table.render(x, y);

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            }) => {
                if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                    break; // Handle Ctrl+C gracefully
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                table.move_cursor_up();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                table.move_cursor_down();
            }

            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            _ => {}
        }
        table.render(x, y);
        // Clear the screen before each render to avoid duplicate lines
    }

    // Ensure the terminal is properly reset on exit
    println!("Exiting gracefully...");
}
