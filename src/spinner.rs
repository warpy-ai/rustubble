use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, style};
use lazy_static::lazy_static;
use maplit::{self, hashmap};
use std::collections::HashMap;
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

pub struct SpinnerData {
    pub frames: Vec<&'static str>,
    pub interval: u64, // Milliseconds between frame updates
}

lazy_static! {
    static ref SPINNERS: HashMap<String, SpinnerData> = {
        hashmap! {
            "Dots2".into() => SpinnerData {frames: vec![
              "⣾",
              "⣽",
              "⣻",
              "⢿",
              "⡿",
              "⣟",
              "⣯",
              "⣷"
            ], interval: 80},
            "Dots2".into() => SpinnerData {frames: vec![
                "⣾",
                "⣽",
                "⣻",
                "⢿",
                "⡿",
                "⣟",
                "⣯",
                "⣷"
              ], interval: 80},
              "Dots3".into() => SpinnerData {frames: vec![
                "⠋",
                "⠙",
                "⠚",
                "⠞",
                "⠖",
                "⠦",
                "⠴",
                "⠲",
                "⠳",
                "⠓"
              ], interval: 80},
              "Dots4".into() => SpinnerData {frames: vec![
                "⠄",
                "⠆",
                "⠇",
                "⠋",
                "⠙",
                "⠸",
                "⠰",
                "⠠",
                "⠰",
                "⠸",
                "⠙",
                "⠋",
                "⠇",
                "⠆"
              ], interval: 80},
              "Dots5".into() => SpinnerData {frames: vec![
                "⠋",
                "⠙",
                "⠚",
                "⠒",
                "⠂",
                "⠂",
                "⠒",
                "⠲",
                "⠴",
                "⠦",
                "⠖",
                "⠒",
                "⠐",
                "⠐",
                "⠒",
                "⠓",
                "⠋"
              ], interval: 80},
              "Dots6".into() => SpinnerData {frames: vec![
                "⠁",
                "⠉",
                "⠙",
                "⠚",
                "⠒",
                "⠂",
                "⠂",
                "⠒",
                "⠲",
                "⠴",
                "⠤",
                "⠄",
                "⠄",
                "⠤",
                "⠴",
                "⠲",
                "⠒",
                "⠂",
                "⠂",
                "⠒",
                "⠚",
                "⠙",
                "⠉",
                "⠁"
              ], interval: 80},
              "Dots7".into() => SpinnerData {frames: vec![
                "⠈",
                "⠉",
                "⠋",
                "⠓",
                "⠒",
                "⠐",
                "⠐",
                "⠒",
                "⠖",
                "⠦",
                "⠤",
                "⠠",
                "⠠",
                "⠤",
                "⠦",
                "⠖",
                "⠒",
                "⠐",
                "⠐",
                "⠒",
                "⠓",
                "⠋",
                "⠉",
                "⠈"
              ], interval: 80},
              "Dots8".into() => SpinnerData {frames: vec![
                "⠁",
                "⠁",
                "⠉",
                "⠙",
                "⠚",
                "⠒",
                "⠂",
                "⠂",
                "⠒",
                "⠲",
                "⠴",
                "⠤",
                "⠄",
                "⠄",
                "⠤",
                "⠠",
                "⠠",
                "⠤",
                "⠦",
                "⠖",
                "⠒",
                "⠐",
                "⠐",
                "⠒",
                "⠓",
                "⠋",
                "⠉",
                "⠈",
                "⠈"
              ], interval: 80},
              "Dots9".into() => SpinnerData {frames: vec![
                "⢹",
                "⢺",
                "⢼",
                "⣸",
                "⣇",
                "⡧",
                "⡗",
                "⡏"
              ], interval: 80},
              "Dots10".into() => SpinnerData {frames: vec![
                "⢄",
                "⢂",
                "⢁",
                "⡁",
                "⡈",
                "⡐",
                "⡠"
              ], interval: 80},
              "Dots11".into() => SpinnerData {frames: vec![
                "⠁",
                "⠂",
                "⠄",
                "⡀",
                "⢀",
                "⠠",
                "⠐",
                "⠈"
              ], interval: 100},
              "Dots12".into() => SpinnerData {frames: vec![
                "⢀⠀",
                "⡀⠀",
                "⠄⠀",
                "⢂⠀",
                "⡂⠀",
                "⠅⠀",
                "⢃⠀",
                "⡃⠀",
                "⠍⠀",
                "⢋⠀",
                "⡋⠀",
                "⠍⠁",
                "⢋⠁",
                "⡋⠁",
                "⠍⠉",
                "⠋⠉",
                "⠋⠉",
                "⠉⠙",
                "⠉⠙",
                "⠉⠩",
                "⠈⢙",
                "⠈⡙",
                "⢈⠩",
                "⡀⢙",
                "⠄⡙",
                "⢂⠩",
                "⡂⢘",
                "⠅⡘",
                "⢃⠨",
                "⡃⢐",
                "⠍⡐",
                "⢋⠠",
                "⡋⢀",
                "⠍⡁",
                "⢋⠁",
                "⡋⠁",
                "⠍⠉",
                "⠋⠉",
                "⠋⠉",
                "⠉⠙",
                "⠉⠙",
                "⠉⠩",
                "⠈⢙",
                "⠈⡙",
                "⠈⠩",
                "⠀⢙",
                "⠀⡙",
                "⠀⠩",
                "⠀⢘",
                "⠀⡘",
                "⠀⠨",
                "⠀⢐",
                "⠀⡐",
                "⠀⠠",
                "⠀⢀",
                "⠀⡀"
              ], interval: 80},
              "Dots8Bit".into() => SpinnerData {frames: vec![
                "⠀",
                "⠁",
                "⠂",
                "⠃",
                "⠄",
                "⠅",
                "⠆",
                "⠇",
                "⡀",
                "⡁",
                "⡂",
                "⡃",
                "⡄",
                "⡅",
                "⡆",
                "⡇",
                "⠈",
                "⠉",
                "⠊",
                "⠋",
                "⠌",
                "⠍",
                "⠎",
                "⠏",
                "⡈",
                "⡉",
                "⡊",
                "⡋",
                "⡌",
                "⡍",
                "⡎",
                "⡏",
                "⠐",
                "⠑",
                "⠒",
                "⠓",
                "⠔",
                "⠕",
                "⠖",
                "⠗",
                "⡐",
                "⡑",
                "⡒",
                "⡓",
                "⡔",
                "⡕",
                "⡖",
                "⡗",
                "⠘",
                "⠙",
                "⠚",
                "⠛",
                "⠜",
                "⠝",
                "⠞",
                "⠟",
                "⡘",
                "⡙",
                "⡚",
                "⡛",
                "⡜",
                "⡝",
                "⡞",
                "⡟",
                "⠠",
                "⠡",
                "⠢",
                "⠣",
                "⠤",
                "⠥",
                "⠦",
                "⠧",
                "⡠",
                "⡡",
                "⡢",
                "⡣",
                "⡤",
                "⡥",
                "⡦",
                "⡧",
                "⠨",
                "⠩",
                "⠪",
                "⠫",
                "⠬",
                "⠭",
                "⠮",
                "⠯",
                "⡨",
                "⡩",
                "⡪",
                "⡫",
                "⡬",
                "⡭",
                "⡮",
                "⡯",
                "⠰",
                "⠱",
                "⠲",
                "⠳",
                "⠴",
                "⠵",
                "⠶",
                "⠷",
                "⡰",
                "⡱",
                "⡲",
                "⡳",
                "⡴",
                "⡵",
                "⡶",
                "⡷",
                "⠸",
                "⠹",
                "⠺",
                "⠻",
                "⠼",
                "⠽",
                "⠾",
                "⠿",
                "⡸",
                "⡹",
                "⡺",
                "⡻",
                "⡼",
                "⡽",
                "⡾",
                "⡿",
                "⢀",
                "⢁",
                "⢂",
                "⢃",
                "⢄",
                "⢅",
                "⢆",
                "⢇",
                "⣀",
                "⣁",
                "⣂",
                "⣃",
                "⣄",
                "⣅",
                "⣆",
                "⣇",
                "⢈",
                "⢉",
                "⢊",
                "⢋",
                "⢌",
                "⢍",
                "⢎",
                "⢏",
                "⣈",
                "⣉",
                "⣊",
                "⣋",
                "⣌",
                "⣍",
                "⣎",
                "⣏",
                "⢐",
                "⢑",
                "⢒",
                "⢓",
                "⢔",
                "⢕",
                "⢖",
                "⢗",
                "⣐",
                "⣑",
                "⣒",
                "⣓",
                "⣔",
                "⣕",
                "⣖",
                "⣗",
                "⢘",
                "⢙",
                "⢚",
                "⢛",
                "⢜",
                "⢝",
                "⢞",
                "⢟",
                "⣘",
                "⣙",
                "⣚",
                "⣛",
                "⣜",
                "⣝",
                "⣞",
                "⣟",
                "⢠",
                "⢡",
                "⢢",
                "⢣",
                "⢤",
                "⢥",
                "⢦",
                "⢧",
                "⣠",
                "⣡",
                "⣢",
                "⣣",
                "⣤",
                "⣥",
                "⣦",
                "⣧",
                "⢨",
                "⢩",
                "⢪",
                "⢫",
                "⢬",
                "⢭",
                "⢮",
                "⢯",
                "⣨",
                "⣩",
                "⣪",
                "⣫",
                "⣬",
                "⣭",
                "⣮",
                "⣯",
                "⢰",
                "⢱",
                "⢲",
                "⢳",
                "⢴",
                "⢵",
                "⢶",
                "⢷",
                "⣰",
                "⣱",
                "⣲",
                "⣳",
                "⣴",
                "⣵",
                "⣶",
                "⣷",
                "⢸",
                "⢹",
                "⢺",
                "⢻",
                "⢼",
                "⢽",
                "⢾",
                "⢿",
                "⣸",
                "⣹",
                "⣺",
                "⣻",
                "⣼",
                "⣽",
                "⣾",
                "⣿"
              ], interval: 80},
              "Line".into() => SpinnerData {frames: vec![
                "-",
                "\\",
                "|",
                "/"
              ], interval: 130},
              "Line2".into() => SpinnerData {frames: vec![
                "⠂",
                "-",
                "–",
                "—",
                "–",
                "-"
              ], interval: 100},
              "Pipe".into() => SpinnerData {frames: vec![
                "┤",
                "┘",
                "┴",
                "└",
                "├",
                "┌",
                "┬",
                "┐"
              ], interval: 100},
              "SimpleDots".into() => SpinnerData {frames: vec![
                ".  ",
                ".. ",
                "...",
                "   "
              ], interval: 400},
              "SimpleDotsScrolling".into() => SpinnerData {frames: vec![
                ".  ",
                ".. ",
                "...",
                " ..",
                "  .",
                "   "
              ], interval: 200},
              "Star".into() => SpinnerData {frames: vec![
                "✶",
                "✸",
                "✹",
                "✺",
                "✹",
                "✷"
              ], interval: 70},
              "Star2".into() => SpinnerData {frames: vec![
                "+",
                "x",
                "*"
              ], interval: 80},
              "Flip".into() => SpinnerData {frames: vec![
                "_",
                "_",
                "_",
                "-",
                "`",
                "`",
                "'",
                "´",
                "-",
                "_",
                "_",
                "_"
              ], interval: 70},
              "Hamburger".into() => SpinnerData {frames: vec![
                "☱",
                "☲",
                "☴"
              ], interval: 100},
              "GrowVertical".into() => SpinnerData {frames: vec![
                "▁",
                "▃",
                "▄",
                "▅",
                "▆",
                "▇",
                "▆",
                "▅",
                "▄",
                "▃"
              ], interval: 120},
              "GrowHorizontal".into() => SpinnerData {frames: vec![
                "▏",
                "▎",
                "▍",
                "▌",
                "▋",
                "▊",
                "▉",
                "▊",
                "▋",
                "▌",
                "▍",
                "▎"
              ], interval: 120},
              "Balloon".into() => SpinnerData {frames: vec![
                " ",
                ".",
                "o",
                "O",
                "@",
                "*",
                " "
              ], interval: 140},
              "Balloon2".into() => SpinnerData {frames: vec![
                ".",
                "o",
                "O",
                "°",
                "O",
                "o",
                "."
              ], interval: 120},
              "Noise".into() => SpinnerData {frames: vec![
                "▓",
                "▒",
                "░"
              ], interval: 100},
              "Bounce".into() => SpinnerData {frames: vec![
                "⠁",
                "⠂",
                "⠄",
                "⠂"
              ], interval: 120},
              "BoxBounce".into() => SpinnerData {frames: vec![
                "▖",
                "▘",
                "▝",
                "▗"
              ], interval: 120},
              "BoxBounce2".into() => SpinnerData {frames: vec![
                "▌",
                "▀",
                "▐",
                "▄"
              ], interval: 100},
              "Triangle".into() => SpinnerData {frames: vec![
                "◢",
                "◣",
                "◤",
                "◥"
              ], interval: 50},
              "Arc".into() => SpinnerData {frames: vec![
                "◜",
                "◠",
                "◝",
                "◞",
                "◡",
                "◟"
              ], interval: 100},
              "Circle".into() => SpinnerData {frames: vec![
                "◡",
                "⊙",
                "◠"
              ], interval: 120},
              "SquareCorners".into() => SpinnerData {frames: vec![
                "◰",
                "◳",
                "◲",
                "◱"
              ], interval: 180},
              "CircleQuarters".into() => SpinnerData {frames: vec![
                "◴",
                "◷",
                "◶",
                "◵"
              ], interval: 120},
              "CircleHalves".into() => SpinnerData {frames: vec![
                "◐",
                "◓",
                "◑",
                "◒"
              ], interval: 50},
              "Squish".into() => SpinnerData {frames: vec![
                "╫",
                "╪"
              ], interval: 100},
              "Toggle".into() => SpinnerData {frames: vec![
                "⊶",
                "⊷"
              ], interval: 250},
              "Toggle2".into() => SpinnerData {frames: vec![
                "▫",
                "▪"
              ], interval: 80},
              "Toggle3".into() => SpinnerData {frames: vec![
                "□",
                "■"
              ], interval: 120},
              "Toggle4".into() => SpinnerData {frames: vec![
                "■",
                "□",
                "▪",
                "▫"
              ], interval: 100},
              "Toggle5".into() => SpinnerData {frames: vec![
                "▮",
                "▯"
              ], interval: 100},
              "Toggle6".into() => SpinnerData {frames: vec![
                "ဝ",
                "၀"
              ], interval: 300},
              "Toggle7".into() => SpinnerData {frames: vec![
                "⦾",
                "⦿"
              ], interval: 80},
              "Toggle8".into() => SpinnerData {frames: vec![
                "◍",
                "◌"
              ], interval: 100},
              "Toggle9".into() => SpinnerData {frames: vec![
                "◉",
                "◎"
              ], interval: 100},
              "Toggle10".into() => SpinnerData {frames: vec![
                "㊂",
                "㊀",
                "㊁"
              ], interval: 100},
              "Toggle11".into() => SpinnerData {frames: vec![
                "⧇",
                "⧆"
              ], interval: 50},
              "Toggle12".into() => SpinnerData {frames: vec![
                "☗",
                "☖"
              ], interval: 120},
              "Toggle13".into() => SpinnerData {frames: vec![
                "=",
                "*",
                "-"
              ], interval: 80},
              "Arrow".into() => SpinnerData {frames: vec![
                "←",
                "↖",
                "↑",
                "↗",
                "→",
                "↘",
                "↓",
                "↙"
              ], interval: 100},
              "Arrow2".into() => SpinnerData {frames: vec![
                "⬆️ ",
                "↗️ ",
                "➡️ ",
                "↘️ ",
                "⬇️ ",
                "↙️ ",
                "⬅️ ",
                "↖️ "
              ], interval: 80},
              "Arrow3".into() => SpinnerData {frames: vec![
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸"
              ], interval: 120},
              "BouncingBar".into() => SpinnerData {frames: vec![
                "[    ]",
                "[=   ]",
                "[==  ]",
                "[=== ]",
                "[ ===]",
                "[  ==]",
                "[   =]",
                "[    ]",
                "[   =]",
                "[  ==]",
                "[ ===]",
                "[====]",
                "[=== ]",
                "[==  ]",
                "[=   ]"
              ], interval: 80},
              "BouncingBall".into() => SpinnerData {frames: vec![
                "( ●    )",
                "(  ●   )",
                "(   ●  )",
                "(    ● )",
                "(     ●)",
                "(    ● )",
                "(   ●  )",
                "(  ●   )",
                "( ●    )",
                "(●     )"
              ], interval: 80},
              "Smiley".into() => SpinnerData {frames: vec![
                "😄 ",
                "😝 "
              ], interval: 200},
              "Monkey".into() => SpinnerData {frames: vec![
                "🙈 ",
                "🙈 ",
                "🙉 ",
                "🙊 "
              ], interval: 300},
              "Hearts".into() => SpinnerData {frames: vec![
                "💛 ",
                "💙 ",
                "💜 ",
                "💚 ",
                "❤️ "
              ], interval: 100},
              "Clock".into() => SpinnerData {frames: vec![
                "🕛 ",
                "🕐 ",
                "🕑 ",
                "🕒 ",
                "🕓 ",
                "🕔 ",
                "🕕 ",
                "🕖 ",
                "🕗 ",
                "🕘 ",
                "🕙 ",
                "🕚 "
              ], interval: 100},
              "Earth".into() => SpinnerData {frames: vec![
                "🌍 ",
                "🌎 ",
                "🌏 "
              ], interval: 180},
              "Material".into() => SpinnerData {frames: vec![
                "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "████████▁▁▁▁▁▁▁▁▁▁▁▁",
                "█████████▁▁▁▁▁▁▁▁▁▁▁",
                "█████████▁▁▁▁▁▁▁▁▁▁▁",
                "██████████▁▁▁▁▁▁▁▁▁▁",
                "███████████▁▁▁▁▁▁▁▁▁",
                "█████████████▁▁▁▁▁▁▁",
                "██████████████▁▁▁▁▁▁",
                "██████████████▁▁▁▁▁▁",
                "▁██████████████▁▁▁▁▁",
                "▁██████████████▁▁▁▁▁",
                "▁██████████████▁▁▁▁▁",
                "▁▁██████████████▁▁▁▁",
                "▁▁▁██████████████▁▁▁",
                "▁▁▁▁█████████████▁▁▁",
                "▁▁▁▁██████████████▁▁",
                "▁▁▁▁██████████████▁▁",
                "▁▁▁▁▁██████████████▁",
                "▁▁▁▁▁██████████████▁",
                "▁▁▁▁▁██████████████▁",
                "▁▁▁▁▁▁██████████████",
                "▁▁▁▁▁▁██████████████",
                "▁▁▁▁▁▁▁█████████████",
                "▁▁▁▁▁▁▁█████████████",
                "▁▁▁▁▁▁▁▁████████████",
                "▁▁▁▁▁▁▁▁████████████",
                "▁▁▁▁▁▁▁▁▁███████████",
                "▁▁▁▁▁▁▁▁▁███████████",
                "▁▁▁▁▁▁▁▁▁▁██████████",
                "▁▁▁▁▁▁▁▁▁▁██████████",
                "▁▁▁▁▁▁▁▁▁▁▁▁████████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
                "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
                "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
                "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
                "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
                "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
                "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
                "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
                "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
                "████████▁▁▁▁▁▁▁▁▁▁▁▁",
                "█████████▁▁▁▁▁▁▁▁▁▁▁",
                "█████████▁▁▁▁▁▁▁▁▁▁▁",
                "█████████▁▁▁▁▁▁▁▁▁▁▁",
                "█████████▁▁▁▁▁▁▁▁▁▁▁",
                "███████████▁▁▁▁▁▁▁▁▁",
                "████████████▁▁▁▁▁▁▁▁",
                "████████████▁▁▁▁▁▁▁▁",
                "██████████████▁▁▁▁▁▁",
                "██████████████▁▁▁▁▁▁",
                "▁██████████████▁▁▁▁▁",
                "▁██████████████▁▁▁▁▁",
                "▁▁▁█████████████▁▁▁▁",
                "▁▁▁▁▁████████████▁▁▁",
                "▁▁▁▁▁████████████▁▁▁",
                "▁▁▁▁▁▁███████████▁▁▁",
                "▁▁▁▁▁▁▁▁█████████▁▁▁",
                "▁▁▁▁▁▁▁▁█████████▁▁▁",
                "▁▁▁▁▁▁▁▁▁█████████▁▁",
                "▁▁▁▁▁▁▁▁▁█████████▁▁",
                "▁▁▁▁▁▁▁▁▁▁█████████▁",
                "▁▁▁▁▁▁▁▁▁▁▁████████▁",
                "▁▁▁▁▁▁▁▁▁▁▁████████▁",
                "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
                "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
                "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁"
              ], interval: 17},
              "Moon".into() => SpinnerData {frames: vec![
                "🌑 ",
                "🌒 ",
                "🌓 ",
                "🌔 ",
                "🌕 ",
                "🌖 ",
                "🌗 ",
                "🌘 "
              ], interval: 80},
              "Runner".into() => SpinnerData {frames: vec![
                "🚶 ",
                "🏃 "
              ], interval: 140},
              "Pong".into() => SpinnerData {frames: vec![
                "▐⠂       ▌",
                "▐⠈       ▌",
                "▐ ⠂      ▌",
                "▐ ⠠      ▌",
                "▐  ⡀     ▌",
                "▐  ⠠     ▌",
                "▐   ⠂    ▌",
                "▐   ⠈    ▌",
                "▐    ⠂   ▌",
                "▐    ⠠   ▌",
                "▐     ⡀  ▌",
                "▐     ⠠  ▌",
                "▐      ⠂ ▌",
                "▐      ⠈ ▌",
                "▐       ⠂▌",
                "▐       ⠠▌",
                "▐       ⡀▌",
                "▐      ⠠ ▌",
                "▐      ⠂ ▌",
                "▐     ⠈  ▌",
                "▐     ⠂  ▌",
                "▐    ⠠   ▌",
                "▐    ⡀   ▌",
                "▐   ⠠    ▌",
                "▐   ⠂    ▌",
                "▐  ⠈     ▌",
                "▐  ⠂     ▌",
                "▐ ⠠      ▌",
                "▐ ⡀      ▌",
                "▐⠠       ▌"
              ], interval: 80},
              "Shark".into() => SpinnerData {frames: vec![
                "▐|\\____________▌",
                "▐_|\\___________▌",
                "▐__|\\__________▌",
                "▐___|\\_________▌",
                "▐____|\\________▌",
                "▐_____|\\_______▌",
                "▐______|\\______▌",
                "▐_______|\\_____▌",
                "▐________|\\____▌",
                "▐_________|\\___▌",
                "▐__________|\\__▌",
                "▐___________|\\_▌",
                "▐____________|\\▌",
                "▐____________/|▌",
                "▐___________/|_▌",
                "▐__________/|__▌",
                "▐_________/|___▌",
                "▐________/|____▌",
                "▐_______/|_____▌",
                "▐______/|______▌",
                "▐_____/|_______▌",
                "▐____/|________▌",
                "▐___/|_________▌",
                "▐__/|__________▌",
                "▐_/|___________▌",
                "▐/|____________▌"
              ], interval: 120},
              "Dqpb".into() => SpinnerData {frames: vec![
                "d",
                "q",
                "p",
                "b"
              ], interval: 100},
              "Weather".into() => SpinnerData {frames: vec![
                "☀️ ",
                "☀️ ",
                "☀️ ",
                "🌤 ",
                "⛅️ ",
                "🌥 ",
                "☁️ ",
                "🌧 ",
                "🌨 ",
                "🌧 ",
                "🌨 ",
                "🌧 ",
                "🌨 ",
                "⛈ ",
                "🌨 ",
                "🌧 ",
                "🌨 ",
                "☁️ ",
                "🌥 ",
                "⛅️ ",
                "🌤 ",
                "☀️ ",
                "☀️ "
              ], interval: 100},
              "Christmas".into() => SpinnerData {frames: vec![
                "🌲",
                "🎄"
              ], interval: 400},
              "Grenade".into() => SpinnerData {frames: vec![
                "،  ",
                "′  ",
                " ´ ",
                " ‾ ",
                "  ⸌",
                "  ⸊",
                "  |",
                "  ⁎",
                "  ⁕",
                " ෴ ",
                "  ⁓",
                "   ",
                "   ",
                "   "
              ], interval: 80},
              "Point".into() => SpinnerData {frames: vec![
                "∙∙∙",
                "●∙∙",
                "∙●∙",
                "∙∙●",
                "∙∙∙"
              ], interval: 125},
              "Layer".into() => SpinnerData {frames: vec![
                "-",
                "=",
                "≡"
              ], interval: 150},
              "BetaWave".into() => SpinnerData {frames: vec![
                "ρββββββ",
                "βρβββββ",
                "ββρββββ",
                "βββρβββ",
                "ββββρββ",
                "βββββρβ",
                "ββββββρ"
              ], interval: 80},
              "FingerDance".into() => SpinnerData {frames: vec![
                "🤘 ",
                "🤟 ",
                "🖖 ",
                "✋ ",
                "🤚 ",
                "👆 "
              ], interval: 160},
              "FistBump".into() => SpinnerData {frames: vec![
                "🤜　　　　🤛 ",
                "🤜　　　　🤛 ",
                "🤜　　　　🤛 ",
                "　🤜　　🤛　 ",
                "　　🤜🤛　　 ",
                "　🤜✨🤛　　 ",
                "🤜　✨　🤛　 "
              ], interval: 80},
              "SoccerHeader".into() => SpinnerData {frames: vec![
                " 🧑⚽️       🧑 ",
                "🧑  ⚽️      🧑 ",
                "🧑   ⚽️     🧑 ",
                "🧑    ⚽️    🧑 ",
                "🧑     ⚽️   🧑 ",
                "🧑      ⚽️  🧑 ",
                "🧑       ⚽️🧑  ",
                "🧑      ⚽️  🧑 ",
                "🧑     ⚽️   🧑 ",
                "🧑    ⚽️    🧑 ",
                "🧑   ⚽️     🧑 ",
                "🧑  ⚽️      🧑 "
              ], interval: 80},
              "Mindblown".into() => SpinnerData {frames: vec![
                "😐 ",
                "😐 ",
                "😮 ",
                "😮 ",
                "😦 ",
                "😦 ",
                "😧 ",
                "😧 ",
                "🤯 ",
                "💥 ",
                "✨ ",
                "　 ",
                "　 ",
                "　 "
              ], interval: 160},
              "Speaker".into() => SpinnerData {frames: vec![
                "🔈 ",
                "🔉 ",
                "🔊 ",
                "🔉 "
              ], interval: 160},
              "OrangePulse".into() => SpinnerData {frames: vec![
                "🔸 ",
                "🔶 ",
                "🟠 ",
                "🟠 ",
                "🔶 "
              ], interval: 100},
              "BluePulse".into() => SpinnerData {frames: vec![
                "🔹 ",
                "🔷 ",
                "🔵 ",
                "🔵 ",
                "🔷 "
              ], interval: 100},
              "OrangeBluePulse".into() => SpinnerData {frames: vec![
                "🔸 ",
                "🔶 ",
                "🟠 ",
                "🟠 ",
                "🔶 ",
                "🔹 ",
                "🔷 ",
                "🔵 ",
                "🔵 ",
                "🔷 "
              ], interval: 100},
              "TimeTravel".into() => SpinnerData {frames: vec![
                "🕛 ",
                "🕚 ",
                "🕙 ",
                "🕘 ",
                "🕗 ",
                "🕖 ",
                "🕕 ",
                "🕔 ",
                "🕓 ",
                "🕒 ",
                "🕑 ",
                "🕐 "
              ], interval: 100},
              "Aesthetic".into() => SpinnerData {frames: vec![
                "▰▱▱▱▱▱▱",
                "▰▰▱▱▱▱▱",
                "▰▰▰▱▱▱▱",
                "▰▰▰▰▱▱▱",
                "▰▰▰▰▰▱▱",
                "▰▰▰▰▰▰▱",
                "▰▰▰▰▰▰▰",
                "▰▱▱▱▱▱▱"
              ], interval: 80}
        }
    };
}

pub struct Spinner {
    is_running: Arc<Mutex<bool>>,
    frames: Arc<Vec<&'static str>>,
    current_frame: Arc<Mutex<usize>>,
    color: Color,
    message: String,
}

impl Spinner {
    pub fn new(color: Color, message: String, style: &str) -> Self {
        let spinner_data = SPINNERS.get(style).unwrap();
        Self {
            is_running: Arc::new(Mutex::new(false)),
            frames: Arc::new(spinner_data.frames.clone()),
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

        let full_display = format!("{}  {}", frame, message);
        execute!(
            stdout,
            MoveTo(x, y),
            Clear(ClearType::CurrentLine),
            Print(color_to_ansi(color)),
            Print(full_display),
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
