use crate::command::CommandInfo;
use crate::help::HelpComponent;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState},
};
use std::io;

#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub subtitle: String,
}

pub struct ItemList {
    pub title: String,
    pub items: Vec<Item>,
    state: ListState,
    pub filter: String,
    filtered_items: Vec<Item>,
    pub showing_filter: bool,
}

impl ItemList {
    pub fn new(title: String, items: Vec<Item>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        ItemList {
            title,
            items: items.clone(),
            state,
            filter: String::new(),
            filtered_items: items,
            showing_filter: false,
        }
    }

    // Add these methods
    pub fn update_filter(&mut self) {
        self.filtered_items = self
            .items
            .iter()
            .filter(|item| {
                item.title
                    .to_lowercase()
                    .contains(&self.filter.to_lowercase())
            })
            .cloned()
            .collect();
        self.state.select(Some(0));
    }

    pub fn get_selected_item(&self) -> Option<&Item> {
        self.state
            .selected()
            .and_then(|i| self.filtered_items.get(i))
    }
    // Update the render method
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        println!("Rendering ItemList"); // Debug print
        println!("showing_filter: {}", self.showing_filter); // Debug print
        println!("filter: {}", self.filter); // Debug print

        let items: Vec<ListItem> = self
            .filtered_items
            .iter()
            .enumerate()
            .map(|(index, i)| {
                let (title_style, subtitle_style, prefix) = if Some(index) == self.state.selected()
                {
                    (
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                        Style::default().fg(Color::Magenta).italic(),
                        "│",
                    )
                } else {
                    (
                        Style::default().fg(Color::White),
                        Style::default().fg(Color::Gray),
                        " ",
                    )
                };

                ListItem::new(vec![
                    Line::from(vec![Span::raw(prefix)]), // Top padding (reduced to 1 line)
                    Line::from(vec![
                        Span::raw(prefix),
                        Span::raw(" "),
                        Span::styled(&i.title, title_style),
                    ]),
                    Line::from(vec![
                        Span::raw(prefix),
                        Span::raw(" "),
                        Span::styled(&i.subtitle, subtitle_style),
                    ]),
                    Line::from(vec![Span::raw(prefix)]), // Bottom padding (reduced to 1 line)
                ])
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default())
            .highlight_symbol("");

        let mut block = Block::default().padding(ratatui::widgets::Padding::new(2, 2, 1, 1));

        // Update this part to show the filter
        let title = if self.showing_filter {
            format!("{} | Filter: {}", self.title, self.filter)
        } else {
            self.title.clone()
        };

        println!("Block title: {}", title); // Debug print

        block = block.title(title);

        let padded_list = list.block(block);
        f.render_stateful_widget(padded_list, area, &mut self.state);
    } // Add this closing brace

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.filtered_items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + self.filtered_items.len() - 1) % self.filtered_items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn handle_list(list: &mut ItemList, x: u16, y: u16) -> Option<String> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    crossterm::terminal::enable_raw_mode().unwrap();

    let commands = vec![
        CommandInfo::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        CommandInfo::new(KeyCode::Char('q'), KeyModifiers::NONE),
        CommandInfo::new(KeyCode::Char('/'), KeyModifiers::NONE),
        CommandInfo::new(KeyCode::Enter, KeyModifiers::NONE),
        CommandInfo::new(KeyCode::Down, KeyModifiers::NONE),
        CommandInfo::new(KeyCode::Up, KeyModifiers::NONE),
    ];

    let filter_commands = vec![
        CommandInfo::new(KeyCode::Esc, KeyModifiers::NONE),
        CommandInfo::new(KeyCode::Enter, KeyModifiers::NONE),
    ];

    let mut help_component = HelpComponent::new(commands, filter_commands);

    let result = loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                    .split(f.size());

                list.render(f, chunks[0]);
                f.render_widget(help_component.clone(), chunks[1]);
            })
            .unwrap();

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read().unwrap()
        {
            match code {
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break None,
                KeyCode::Char('q') => break None,
                KeyCode::Char('/') => {
                    list.showing_filter = true;
                    list.filter.clear();
                    help_component.activate_filter_mode();
                }
                KeyCode::Esc => {
                    list.showing_filter = false;
                    list.filter.clear();
                    list.update_filter();
                    help_component.deactivate_filter_mode();
                }
                KeyCode::Enter => {
                    if let Some(item) = list.get_selected_item() {
                        break Some(item.title.clone());
                    }
                }
                KeyCode::Down => list.next(),
                KeyCode::Up => list.previous(),
                KeyCode::Char(c) if list.showing_filter => {
                    list.filter.push(c);
                    list.update_filter();
                }
                KeyCode::Backspace if list.showing_filter => {
                    list.filter.pop();
                    list.update_filter();
                }
                _ => {}
            }
        }

        println!("After key handling:"); // Debug print
        println!("showing_filter: {}", list.showing_filter); // Debug print
        println!("filter: {}", list.filter); // Debug print
    };

    crossterm::terminal::disable_raw_mode().unwrap();

    terminal.clear().unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_correctly() {
        let items = vec![
            Item {
                title: "Pocky".into(),
                subtitle: "Expensive".into(),
            },
            Item {
                title: "Ginger".into(),
                subtitle: "Exquisite".into(),
            },
        ];
        let list = ItemList::new("Groceries".into(), items);
        assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn navigates_correctly() {
        let items = vec![
            Item {
                title: "Pocky".into(),
                subtitle: "Expensive".into(),
            },
            Item {
                title: "Ginger".into(),
                subtitle: "Exquisite".into(),
            },
        ];
        let mut list = ItemList::new("Groceries".into(), items.clone());

        list.next();
        assert_eq!(list.state.selected(), Some(1));
        list.next();
        assert_eq!(list.state.selected(), Some(0)); // Loop back to the start
        list.previous();
        assert_eq!(list.state.selected(), Some(1)); // Loop to the end
    }

    #[test]
    fn filters_items() {
        let items = vec![
            Item {
                title: "Pocky".into(),
                subtitle: "Expensive".into(),
            },
            Item {
                title: "Ginger".into(),
                subtitle: "Exquisite".into(),
            },
        ];
        let mut list = ItemList::new("Groceries".into(), items.clone());

        list.filter = "Ginger".to_string();
        list.update_filter();
        assert_eq!(list.filtered_items.len(), 1);
        assert_eq!(list.filtered_items[0].title, "Ginger");

        list.filter = "Pocky".to_string();
        list.update_filter();
        assert_eq!(list.filtered_items.len(), 1);
        assert_eq!(list.filtered_items[0].title, "Pocky");
    }

    #[test]
    fn selects_item_correctly() {
        let items = vec![
            Item {
                title: "Pocky".into(),
                subtitle: "Expensive".into(),
            },
            Item {
                title: "Ginger".into(),
                subtitle: "Exquisite".into(),
            },
        ];
        let mut list = ItemList::new("Groceries".into(), items.clone());

        list.next(); // Move to second item
        let selected = list.get_selected_item();
        assert!(matches!(selected, Some(item) if item.title == "Ginger"));
    }
}
