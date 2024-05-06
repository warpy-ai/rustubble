use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Quit,
    Backspace,
    Delete,
    Help,
    ControlC,
    Enter,
    Filter,
    Esc,
    Up,
    Down,
}

impl Command {
    pub fn describe(key: KeyCode, modifiers: KeyModifiers) -> Self {
        match (key, modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => Command::ControlC,
            (KeyCode::Backspace, _) => Command::Backspace,
            (KeyCode::Delete, _) => Command::Delete,
            (KeyCode::Char('q'), _) => Command::Quit,
            (KeyCode::Char('/'), _) => Command::Filter,
            (KeyCode::Enter, _) => Command::Enter,
            (KeyCode::Up, _) => Command::Up,
            (KeyCode::Down, _) => Command::Down,
            (KeyCode::Esc, _) => Command::Esc,
            // Define other key combinations
            _ => Command::Help, // Default to help or unknown if needed
        }
    }

    pub fn key(&self) -> &'static str {
        match *self {
            Command::Quit => "q",
            Command::Esc => "esc",
            Command::Backspace => "Backspace",
            Command::Delete => "del",
            Command::Help => "h",
            Command::ControlC => "cntrl+c",
            Command::Enter => "\u{2B90}",
            Command::Filter => "/",
            Command::Up => "\u{2191}/h",
            Command::Down => "\u{2193}/l",
            // Additional commands
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Command::Quit => "quit",
            Command::Backspace => "backspace",
            Command::Delete => "delete",
            Command::ControlC => "exit",
            Command::Help => "help",
            Command::Enter => "submit",
            Command::Filter => "filter",
            Command::Up => "up",
            Command::Down => "down",
            Command::Esc => "cancel",
            // Additional commands
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandInfo {
    pub key: String,
    pub modifiers: KeyModifiers,
    pub description: String,
}

impl CommandInfo {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        let command = Command::describe(key, modifiers);
        Self {
            key: command.key().to_string(),
            modifiers,
            description: command.as_str().to_string(),
        }
    }
}
