use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rustubble::list::{handle_list, Item, ItemList};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

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
                title: "Bicoin".to_string(),
                subtitle: "Cheap".to_string(),
            }, // Add more items
        ],
    );

    let (x, y) = (5, 5);
    handle_list(&mut list, x, y);

    disable_raw_mode()
}
