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

pub struct StopWatch {
    start_time: Instant,
    running: bool,
}

impl StopWatch {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            running: true,
        }
    }

    pub fn elapsed(&self) -> Duration {
        if self.running {
            self.start_time.elapsed()
        } else {
            Duration::new(0, 0)
        }
    }
    pub fn toggle(&mut self) {
        if self.running {
            self.start_time = Instant::now() - self.start_time.elapsed();
            self.running = false;
        } else {
            self.start_time = Instant::now() - self.start_time.elapsed();
            self.running = true;
        }
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.running = true;
    }

    fn format_duration(elapsed: Duration) -> String {
        let millis = elapsed.subsec_millis();
        let secs = elapsed.as_secs();
        let minutes = secs / 60;
        let hours = minutes / 60;

        match secs {
            0 => format!("Elapsed: {:03}ms", millis),
            1..=59 => format!("Elapsed: {}.{:03}s", secs, millis),
            60..=3599 => format!("Elapsed: {}:{:02}.{:03}m", minutes, secs % 60, millis),
            _ => format!(
                "Elapsed: {}:{:02}:{:02}.{:03}h",
                hours,
                minutes % 60,
                secs % 60,
                millis
            ),
        }
    }

    fn render<B: Backend>(&self, terminal: &mut Terminal<B>, x: u16, y: u16) {
        terminal
            .draw(|f| {
                let size = f.size();
                let area = Rect::new(x, y, size.width - x, 1);

                let elapsed = self.elapsed();
                let timer_text = StopWatch::format_duration(elapsed);

                let paragraph = Paragraph::new(timer_text).alignment(Alignment::Left);
                f.render_widget(paragraph, area);
            })
            .unwrap();
    }
}

pub fn handle_stopwatch(timer: &mut StopWatch, x: u16, y: u16) {
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();
    let tick_rate = Duration::from_millis(50); // Update every second

    loop {
        timer.render(&mut terminal, x, y);
        if poll(tick_rate).unwrap() {
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
}
