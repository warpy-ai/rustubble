use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use rustubble::list::{Item, ItemList};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a list
    let mut list = ItemList::new(
        "Groceries".to_string(),
        vec![
            Item {
                title: "Pocky".to_string(),
                subtitle: "Expensive".to_string(),
            },
            Item {
                title: "Ginger".to_string(),
                subtitle: "Exquisite".to_string(),
            },
            Item {
                title: "Coke".to_string(),
                subtitle: "Cheap".to_string(),
            },
            Item {
                title: "Sprite".to_string(),
                subtitle: "Cheap".to_string(),
            },
            Item {
                title: "Biscin".to_string(),
                subtitle: "Cheap".to_string(),
            },
        ],
    );

    // Run the main loop
    let res = run_app(&mut terminal, &mut list);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(Some(item)) = res {
        println!("Selected item: {} - {}", item.title, item.subtitle);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    list: &mut ItemList,
) -> io::Result<Option<Item>> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            list.render(f, size);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(None),
                KeyCode::Down => list.next(),
                KeyCode::Up => list.previous(),
                KeyCode::Enter => return Ok(list.get_selected_item().cloned()),
                _ => {}
            }
        }
    }
}
