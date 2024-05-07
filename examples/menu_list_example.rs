use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rustubble::menu_list::{handle_menu_list, Menu};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut new_menu = Menu::new(
        "Main Menu".to_string(),
        "Select an option:".to_string(),
        vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
            "Option 4".to_string(),
        ],
    );

    let (x, y) = (5, 5);

    let selected_menu = handle_menu_list(&mut new_menu, x, y);

    println!("Selected Menu: {:?}", selected_menu);
    disable_raw_mode()
}
