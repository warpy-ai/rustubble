use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn color_to_ansi(color: Color) -> String {
    match color {
        Color::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
        // Handle other color cases if needed
        _ => String::new(), // Default case
    }
}

pub struct Spinner {
    is_running: Arc<Mutex<bool>>,
    frames: Vec<&'static str>,
    current_frame: Arc<Mutex<usize>>,
    color: Color,
    message: String,
}

impl Spinner {
    pub fn new(color: Color, message: String) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            frames: vec!["🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 ", "👌"],
            current_frame: Arc::new(Mutex::new(0)),
            color,
            message,
        }
    }

    pub fn start(&self, x: u16, y: u16) {
        let running = self.is_running.clone();
        let frames = self.frames.clone();
        let current_frame = self.current_frame.clone();
        let color = self.color;
        let message = self.message.clone();

        *running.lock().unwrap() = true;

        thread::spawn(move || {
            while *running.lock().unwrap() {
                // Retrieve the current frame index
                let frame_idx = {
                    let mut frame = current_frame.lock().unwrap();
                    let val = *frame;
                    *frame = (*frame + 1) % frames.len(); // Update frame index
                    val
                };
                // Call the render function with the current state
                Spinner::render(x, y, frames[frame_idx], color, &message);
                thread::sleep(Duration::from_millis(120));
            }
        });
    }

    pub fn stop(&self) {
        let mut running = self.is_running.lock().unwrap();
        *running = false;
    }

    // Moved render back to being an associated function of Spinner for clarity
    fn render(x: u16, y: u16, frame: &str, color: Color, message: &str) {
        let mut stdout = stdout();
        execute!(
            stdout,
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(color_to_ansi(color)),
            Print(frame),
            Print(message),
            ResetColor
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

pub fn handle_spinner(spinner: &Spinner, x: u16, y: u16) {
    spinner.start(x, y);
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            }) => {
                if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                    spinner.stop();
                    break;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                spinner.stop();
                break;
            }
            _ => {}
        }
    }
}
