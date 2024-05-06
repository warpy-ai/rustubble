use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::command::CommandInfo;

#[derive(Clone, Debug)]
pub struct HelpComponent {
    pub normal_commands: Vec<CommandInfo>,
    filter_commands: Option<Vec<CommandInfo>>,
    active_commands: Vec<CommandInfo>,
}

impl HelpComponent {
    pub fn new(normal_commands: Vec<CommandInfo>, filter_commands: Vec<CommandInfo>) -> Self {
        Self {
            normal_commands: normal_commands.clone(),
            filter_commands: Some(filter_commands),
            active_commands: normal_commands, // Start in normal mode by default
        }
    }

    // Method to switch to filter mode
    pub fn activate_filter_mode(&mut self) {
        self.active_commands = self.filter_commands.clone().unwrap();
    }

    // Method to switch to normal mode
    pub fn deactivate_filter_mode(&mut self) {
        self.active_commands = self.normal_commands.clone();
    }
}
impl Widget for HelpComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = Vec::new();

        for (indx, cmd) in self.active_commands.iter().enumerate() {
            spans.push(Span::styled(
                format!("{} ", cmd.key),
                Style::default().fg(Color::DarkGray),
            ));

            spans.push(Span::styled(
                format!("{}", cmd.description),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ));

            if indx < self.active_commands.len() - 1 {
                spans.push(Span::raw(" • "));
            }

            let line = Line::from(spans.clone());
            let text = Text::from(vec![line]);

            let block = Block::default().borders(Borders::NONE);

            let inner_area = block.inner(area);
            block.clone().render(area, buf);

            let paragraph = Paragraph::new(text)
                .block(block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            paragraph.render(inner_area, buf);
        }
    }
}
