use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
    Terminal,
};

use crate::{
    command::{CommandInfo},
    help::HelpComponent,
};

#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub subtitle: String,
}

pub struct ItemList {
    title: String,
    filtered_items: Vec<Item>,
    items: Vec<Item>,
    state: ListState,
    filter: String,
    showing_filter: bool,
}

impl ItemList {
    pub fn new(title: String, items: Vec<Item>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0)); // Initialize the cursor at the first item

        Self {
            title,
            items: items.clone(),
            filtered_items: items,
            state,
            filter: String::new(),
            showing_filter: false,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn get_selected_item(&self) -> Option<&Item> {
        if let Some(selected) = self.state.selected() {
            self.items.get(selected)
        } else {
            None
        }
    }

    pub fn update_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_items = self.items.clone();
        } else {
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
        }
        self.state.select(Some(0)); // Reset selection
    }

    pub fn create_custom_list_item(item: &Item) -> ListItem {
        // Use '\n' to ensure titles and subtitles are on separate lines
        // and ensure that each line is treated as a separate span
        let lines = vec![
            Span::styled(
                " ",
                Style::default()
                    .fg(Color::Gray) // Subtitle can have a different style or color
                    .add_modifier(Modifier::ITALIC),
            ),
            Span::styled(
                &item.title,
                Style::default()
                    .fg(Color::White) // You might adjust color and style as needed
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &item.subtitle,
                Style::default()
                    .fg(Color::Gray) // Subtitle can have a different style or color
                    .add_modifier(Modifier::ITALIC),
            ),
            Span::styled(
                " ",
                Style::default()
                    .fg(Color::Gray) // Subtitle can have a different style or color
                    .add_modifier(Modifier::ITALIC),
            ),
        ];

        // Convert Vec<Span> to Text by wrapping each Span in a Line
        let text = Text::from(
            lines
                .iter()
                .map(|span| Line::from(vec![span.clone()]))
                .collect::<Vec<_>>(),
        );

        ListItem::new(text)
    }

    pub fn render<B: Backend>(
        &self,
        terminal: &mut Terminal<B>,
        rect: Rect,
        help_component: &mut HelpComponent,
    ) {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Percentage(50),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(rect);

                //TODO: add title widget on chunk[0]

                if !self.showing_filter {
                    let title_widget = Paragraph::new(self.title.as_str())
                        .block(Block::default().borders(Borders::NONE));
                    f.render_widget(title_widget, chunks[0]);
                }

                if self.showing_filter {
                    let filter_title = "Filter:";
                    let input = Paragraph::new(format!("{} {}", filter_title, self.filter))
                        .block(Block::default().borders(Borders::NONE));
                    f.render_widget(input, chunks[0]);

                    let cursor_pos = filter_title.len() as u16 + 1 + self.filter.len() as u16; // "Filter: " is 7 chars + 1 space
                    f.set_cursor(chunks[0].x + cursor_pos, chunks[0].y); // +1 because the text starts one line down in the block
                }

                let items: Vec<ListItem> = self
                    .filtered_items
                    .iter()
                    .map(ItemList::create_custom_list_item)
                    .collect();

                let list = List::new(items)
                    .block(Block::default().title("").borders(Borders::NONE))
                    .highlight_style(
                        Style::default()
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("│ ")
                    .repeat_highlight_symbol(true);

                f.render_stateful_widget(list, chunks[1], &mut self.state.clone());

                f.render_widget(help_component.clone(), chunks[2])
            })
            .unwrap();
    }
}

pub fn handle_list(list: &mut ItemList, x: u16, y: u16) -> Option<String> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.clear().unwrap();

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

        if list.showing_filter {
            help_component.activate_filter_mode();
        } else {
            help_component.deactivate_filter_mode();
        }

        list.render(
            &mut terminal,
            Rect::new(x, y, 40, 50),
            &mut help_component.clone(),
        );

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read().unwrap()
        {
            match code {
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    return None;
                }
                KeyCode::Char('/') => {
                    list.showing_filter = !list.showing_filter;
                }
                KeyCode::Esc => list.showing_filter = false,
                KeyCode::Char('q') => return None,
                KeyCode::Down => list.next(),
                KeyCode::Up => list.previous(),
                KeyCode::Enter => {
                    return Some(list.filtered_items[list.state.selected()?].title.clone())
                }
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
        list.render(&mut terminal, Rect::new(x, y, 40, 50), &mut help_component);
    }
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
