use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crate::{command::CommandInfo, help::HelpComponent};

#[derive(Clone, Debug)]
struct MenuItem {
    name: String,
    selected: bool,
}

#[derive(Clone, Debug)]
pub struct Menu {
    title: String,
    subtitle: String,
    items: Vec<MenuItem>,
    selection_state: ListState,
}

impl Menu {
    pub fn new(title: String, subtitle: String, items: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0)); // Initialize the cursor at the first item

        let menu_items = items
            .into_iter()
            .map(|item| MenuItem {
                name: item,
                selected: false,
            })
            .collect();

        Self {
            title,
            subtitle,
            items: menu_items,
            selection_state: state,
        }
    }

    pub fn render<B: Backend>(
        &self,
        terminal: &mut Terminal<B>,
        area: Rect,
        help_component: &mut HelpComponent,
    ) {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(2),
                            Constraint::Max(10),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(area);

                let title_widget = format!("{}", self.title);
                let title = Paragraph::new(title_widget.as_str())
                    .style(Style::default().add_modifier(Modifier::BOLD))
                    .fg(Color::LightMagenta)
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(title, chunks[0]);

                let subtitle_widget = format!("{}", self.subtitle);
                let subtitle = Paragraph::new(subtitle_widget.as_str())
                    .style(
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::DarkGray),
                    )
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(subtitle, chunks[1]);

                let items: Vec<ListItem> = self
                    .items
                    .iter()
                    .map(|item| {
                        let content = if item.selected {
                            format!("✓ {}", item.name)
                        } else {
                            format!("  {}", item.name)
                        };
                        ListItem::new(content)
                    })
                    .collect();

                //TODO: add color to symbol
                let symbol = "> ";
                let list = List::new(items)
                    .block(Block::default().borders(Borders::NONE))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(symbol)
                    .scroll_padding(4);
                f.render_stateful_widget(list, chunks[2], &mut self.selection_state.clone());
                //TODO: calculate the area and render widget help_component under list
                f.render_widget(help_component.clone(), chunks[3]);
            })
            .unwrap();
    }

    pub fn up(&mut self) {
        let i = match self.selection_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selection_state.select(Some(i));
    }

    pub fn down(&mut self) {
        let i = match self.selection_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selection_state.select(Some(i));
    }

    pub fn toggle_selection(&mut self) {
        if let Some(i) = self.selection_state.selected() {
            self.items[i].selected = !self.items[i].selected;
        }
    }

    // Add methods to handle key inputs: up, down, toggle selection, etc.
}

pub fn handle_menu_list(menu: &mut Menu, x: u16, y: u16) -> Option<String> {
    // Render the menu
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    loop {
        terminal.clear().unwrap();

        let commands = vec![
            CommandInfo::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            CommandInfo::new(KeyCode::Char('q'), KeyModifiers::NONE),
            CommandInfo::new(KeyCode::Enter, KeyModifiers::NONE),
            CommandInfo::new(KeyCode::Down, KeyModifiers::NONE),
            CommandInfo::new(KeyCode::Up, KeyModifiers::NONE),
        ];

        let mut help_component = HelpComponent::new(commands, vec![]);

        menu.render(&mut terminal, Rect::new(x, y, 40, 50), &mut help_component);

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            }) => {
                if c == 'j' {
                    menu.down();
                }
                if c == 'k' {
                    menu.up();
                }
                if c == 'q' {
                    return None;
                }
                if modifiers.contains(KeyModifiers::CONTROL) && c == 't' {
                    menu.toggle_selection();
                }
                if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                    return None;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => menu.up(),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => menu.down(),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                if let Some(i) = menu.selection_state.selected() {
                    return Some(menu.items[i].name.clone());
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_list() {
        let mut menu = Menu::new("Title".to_string(), "Subtitle".to_string(), vec![]);
        menu.up();
        menu.down();
        menu.toggle_selection();

        assert_eq!(menu.selection_state.selected(), Some(0));
    }
}
