use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

pub struct Timer {
    start_time: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
        }
    }

    pub fn time_remaining(&self) -> Duration {
        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            Duration::default() // No negative durations, stop at 0
        } else {
            self.duration - elapsed
        }
    }

    fn format_duration(&self) -> String {
        let remaining = self.time_remaining();

        let millis = remaining.subsec_millis();
        let secs = remaining.as_secs();
        let minutes = remaining.as_secs() / 60;
        let hours = remaining.as_secs() / 3600;

        let timer_text = match remaining.as_secs() {
            0 => format!("Exiting in {:03}ms", millis), // Only milliseconds
            1..=59 => format!("Exiting in {}.{:03}s", secs, millis), // Seconds and milliseconds
            60..=3599 => format!("Exiting in {}:{:02}.{:03}m", minutes, secs, millis), // Minutes and seconds
            _ => format!(
                "Exiting in {}:{:02}:{:02}.{:03}h",
                hours, minutes, secs, millis
            ), // Hours and minutes
        };

        timer_text
    }

    fn render<B: Backend>(&self, terminal: &mut Terminal<B>, x: u16, y: u16) {
        terminal
            .draw(|f| {
                let size = f.size();
                let area = Rect::new(x, y, size.width - x, 1);

                let timer_text = self.format_duration();

                let paragraph = Paragraph::new(timer_text).alignment(Alignment::Left);
                f.render_widget(paragraph, area);
            })
            .unwrap();
    }
}

pub fn handle_timer(timer: &mut Timer, x: u16, y: u16) {
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    while timer.time_remaining() > Duration::default() {
        timer.render(&mut terminal, x, y);

        if poll(Duration::from_millis(50)).unwrap() {
            if let Ok(Event::Key(key)) = read() {
                if key.code == KeyCode::Esc
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                    || key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    terminal
        .draw(|f| {
            let size = f.size();
            let area = Rect::new(x, y, size.width - x, 1);

            let paragraph = Paragraph::new("All done").alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        })
        .unwrap();
}
