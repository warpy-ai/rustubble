use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::colors::blend_color;

pub struct ProgressBar {
    progress: f32,
    length: u16,
    start_color: Color,
    end_color: Color,
    prefix: String,
}

impl ProgressBar {
    pub fn new(
        prefix: String,
        progress: f32,
        length: u16,
        start_color: Color,
        end_color: Color,
    ) -> Self {
        Self {
            progress,
            length,
            start_color,
            end_color,
            prefix,
        }
    }

    pub fn update(&mut self, progress: f32, x: u16, y: u16) {
        self.progress = progress;
        self.render(x, y);
    }

    fn render(&self, x: u16, y: u16) {
        let mut stdout = stdout();

        execute!(stdout, MoveTo(x, y), Clear(ClearType::CurrentLine)).unwrap();
        // Hide the cursor to avoid flicker
        execute!(stdout, Hide).unwrap();

        let show_prefix = !self.prefix.is_empty();

        // Build the progress bar string
        for i in 0..self.length {
            if show_prefix {
                let print_prefix = format!("{} ", self.prefix.as_str());
                execute!(stdout, MoveTo(x, y), Print(print_prefix),).unwrap();
            }
            let gradient_ratio = i as f32 / self.length as f32;
            let color = if gradient_ratio < self.progress {
                blend_color(self.start_color, self.end_color, gradient_ratio)
            } else {
                Color::DarkGrey // Background color of the unfilled part
            };

            execute!(
                stdout,
                MoveTo(x + i, y),
                SetForegroundColor(color),
                Print("▇")
            )
            .unwrap();
        }

        let percentage = format!(" {:.0}%", self.progress * 100.0);

        execute!(
            stdout,
            MoveTo(x + self.length + 1, y),
            SetForegroundColor(Color::White),
            Print(percentage),
            ResetColor,
            Hide
        )
        .unwrap();

        stdout.flush().unwrap(); // Ensure all changes are flushed and visible
    }
}

pub fn handle_progress_bar(progress_bar: &mut ProgressBar, progress: f32, x: u16, y: u16) {
    progress_bar.update(progress, x, y);
    // progress_bar.render(x, y);
}
