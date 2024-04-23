use crossterm::{
    cursor::MoveTo, event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind}, execute, style::Print, terminal::{self, Clear, ClearType}
};
use textwrap::{wrap, Options};
use std::io::{stdout, Write};

pub struct Viewport {
    header: String,
    content: String, // Each string represents a line of the content
    padding: usize,
    scroll_offset: usize,
    height: usize, // Visible height of the content area
    width: usize,  // Visible width of the content area
}

impl Viewport {
    pub fn new(header: String, content: String,  height: usize,width: usize , padding: usize) -> Self {
        Self {
            header,
            content,
            scroll_offset: 0,
            padding,
            height,
            width
        }
    }

    pub fn render(&self, x: u16, y: u16) {
        let mut stdout = stdout();

        // Clear the terminal
        execute!(stdout, Clear(ClearType::All)).unwrap();

        // Render the header
        // println!("{}", self.header);
        self.render_header_box(x, y);

        // Render the content with vertical scrolling
        let content_options = Options::new(self.width - (2* self.padding));
        let wrapped_lines: Vec<String> = wrap(&self.content, content_options).into_iter().map(|cow| cow.into_owned()).collect();
        let visible_content = wrapped_lines.iter().skip(self.scroll_offset).take(self.height);

        let mut line_number = y + 4;
        for line in visible_content {
            execute!(
                stdout,
                MoveTo(x, line_number),
                Clear(ClearType::CurrentLine),
                Print(format!("{:<width$}", line, width=self.width)) // Fill line with spaces to width
            )
            .unwrap();
            line_number += 1;
        }

        // Fill the rest of the viewport with blank lines if content is less than viewport height
        while line_number < y + 3 + self.height as u16 {
            execute!(
                stdout,
                MoveTo(x, line_number),
                Clear(ClearType::CurrentLine),
                Print(format!("{:width$}", " ", width=self.width)) // Fill line with spaces to width
            )
            .unwrap();
            line_number += 1;
        }
        // Update and render the footer
        let progress = (self.scroll_offset as f32 / (wrapped_lines.len() - self.height) as f32 * 100.0).min(100.0);
        let horizontal_line = "─".repeat(self.width - 2);
        let footer_with_progress = format!("{} {:.2}%",horizontal_line, progress);
        // println!("{}", footer_with_progress);

        execute!(
            std::io::stdout(),
            MoveTo(x, y + 5 + self.height as u16),
            Clear(ClearType::CurrentLine),
            Print(&footer_with_progress),
        )
        .unwrap();

        self.hide_cursor();
        stdout.flush().unwrap();
    }

    fn render_header_box(&self, x: u16, y: u16) {
        let mut stdout = stdout();
        // get self.header length
        let header_length = self.header.len();
        let top_border = format!(
            "┌{}┐",
            "─".repeat(header_length + 1)
        );

        let bottom_border = format!(
            "└{}┘",
            "─".repeat(header_length + 1)
        );

        let text_content = format!(
            "│ {}│",
            self.header
        );

        execute!(
            stdout,
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(&top_border)
        )
        .unwrap();

        execute!(
            stdout,
            MoveTo(x, y + 1),
            Clear(ClearType::CurrentLine),
            Print(text_content)
        )
        .unwrap();
        
        execute!(
            stdout,
            MoveTo(x, y + 2),
            Clear(ClearType::CurrentLine),
            Print(&bottom_border)
        )
        .unwrap();

       

    }


    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }
    fn scroll_down(&mut self) {
        // Calculate the total number of lines that content can be wrapped into
        let content_options = Options::new(self.width - (2 * self.padding));
        let wrapped_lines: Vec<String> = wrap(&self.content, content_options).into_iter().map(|cow| cow.into_owned()).collect();
        
        // Check if scrolling down is possible by comparing scroll_offset with the number of lines minus the viewport height
        if self.scroll_offset < wrapped_lines.len().saturating_sub(self.height) {
            self.scroll_offset += 1;
        }
    }

    fn page_down(&mut self) {
        let content_options = Options::new(self.width - (2 * self.padding));
        let wrapped_lines: Vec<String> = wrap(&self.content, content_options).into_iter().map(|cow| cow.into_owned()).collect();
        let max_offset = wrapped_lines.len().saturating_sub(self.height);
        
        if self.scroll_offset < max_offset {
            self.scroll_offset = usize::min(self.scroll_offset + self.height, max_offset);
        }
    }

    fn page_up(&mut self) {
        // Scroll up by the height of the viewport
        self.scroll_offset = self.scroll_offset.saturating_sub(self.height);
    }

    fn hide_cursor(&self) {
        print!("\x1B[?25l"); // Hide the cursor
        std::io::stdout().flush().unwrap(); // Ensure the command is applied immediately
    }

    pub fn show_cursor(&self) {
        print!("\x1B[?25h"); // Show the cursor
        std::io::stdout().flush().unwrap(); // Ensure the command is applied immediately
    }

    // Methods to handle user input and scrolling go here...
}

pub fn handle_viewport(viewport: &mut Viewport, x: u16, y: u16) {
    viewport.render(x,y);
    loop {
        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                match event {
                   
                    Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) => match code {
                        KeyCode::Up | KeyCode::Char('k') => viewport.scroll_up(),
                        KeyCode::Down | KeyCode::Char('j') => viewport.scroll_down(),
                        KeyCode::Esc | KeyCode::Char('q') => {
                            break
                        }
                        KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => {
                            break
                        }
                        KeyCode::PageUp => viewport.page_up(),
                        KeyCode::PageDown => viewport.page_down(),
                        _ => {}
                    },
                    Event::Mouse(MouseEvent {
                        kind: MouseEventKind::ScrollUp,
                        ..
                    }) => viewport.scroll_up(),
                    Event::Mouse(MouseEvent {
                        kind: MouseEventKind::ScrollDown,
                        ..
                    }) => viewport.scroll_down(),
                    _ => {}
                }
            }
            viewport.render(x,y);

        }
    }
}
