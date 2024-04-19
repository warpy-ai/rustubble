use std::vec;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

extern crate rustubble;
use rustubble::table::{handle_table, Table};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?; // Enable raw mode for direct terminal manipulation

    let headers = vec![
        "Rank".to_string(),
        "City".to_string(),
        "Country".to_string(),
        "Population".to_string(),
    ];

    let data = vec![
        vec![
            "1".to_string(),
            "Tokyo".to_string(),
            "Japan".to_string(),
            "37,274,000".to_string(),
        ],
        vec![
            "2".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "3".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "4".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "5".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "6".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        vec![
            "7".to_string(),
            "Delhi".to_string(),
            "India".to_string(),
            "32,065,760".to_string(),
        ],
        // Add more rows as necessary
    ];

    let mut table = Table::new(headers, data, 0, 3, 7); // Selected row is 0, padding is 1

    let (x, y) = (5, 5);
    handle_table(&mut table, x, y);

    // Clean up the terminal
    disable_raw_mode()?;
    // Clean up before exiting

    Ok(())
}
