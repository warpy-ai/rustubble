use crossterm::style::{Color, ResetColor, SetForegroundColor, StyledContent, Stylize};
use crossterm::ExecutableCommand;
use std::io::{stdout, Write};

pub mod custom {
    use crossterm::style::Color;

    pub const PURPLE: Color = Color::Rgb {
        r: 122,
        g: 114,
        b: 229,
    };
    pub const DARK_BLUE: Color = Color::Rgb {
        r: 20,
        g: 94,
        b: 152,
    };
    pub const DARK_PINK: Color = Color::Rgb {
        r: 171,
        g: 87,
        b: 151,
    };
    pub const DARK_WHITE: Color = Color::Rgb {
        r: 218,
        g: 208,
        b: 192,
    };
    pub const CYAN: Color = Color::Rgb {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Color = Color::Rgb {
        r: 255,
        g: 0,
        b: 255,
    };
    pub const ORANGE: Color = Color::Rgb {
        r: 255,
        g: 165,
        b: 0,
    };
}

pub fn display_gradient_text(text: &str, start_color: Color, end_color: Color) {
    // Split the text into lines
    let lines: Vec<&str> = text.lines().collect();

    // Apply gradient to each line
    for (i, line) in lines.iter().enumerate() {
        let gradient_ratio = i as f32 / lines.len() as f32;
        let blended_color = blend_color(start_color, end_color, gradient_ratio);
        let mut stdout = stdout();
        stdout.execute(SetForegroundColor(blended_color)).unwrap();
        writeln!(stdout, "{}", line).unwrap();
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn display_color_text(text: &str, color: Color) {
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(color)).unwrap();
    writeln!(stdout, "{}", text).unwrap();
    stdout.execute(ResetColor).unwrap();
}

pub fn return_color_text(text: &str, color: Color) -> StyledContent<String> {
    String::from(text).with(color)
}

pub fn blend_color(start: Color, end: Color, ratio: f32) -> Color {
    if let (
        Color::Rgb {
            r: r1,
            g: g1,
            b: b1,
        },
        Color::Rgb {
            r: r2,
            g: g2,
            b: b2,
        },
    ) = (start, end)
    {
        let blended_r = blend_value(r1, r2, ratio);
        let blended_g = blend_value(g1, g2, ratio);
        let blended_b = blend_value(b1, b2, ratio);
        Color::Rgb {
            r: blended_r,
            g: blended_g,
            b: blended_b,
        }
    } else {
        Color::White
    }
}

fn blend_value(start: u8, end: u8, ratio: f32) -> u8 {
    ((start as f32 * (1.0 - ratio)) + (end as f32 * ratio)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::Color;

    #[test]
    fn test_display_color_text() {
        display_color_text("Hello, World!", Color::Green);
        // Add more tests for different scenarios
    }
}
