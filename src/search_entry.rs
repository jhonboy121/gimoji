use crate::colors::Colors;
use ratatui::{
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub struct SearchEntry<'c> {
    text: String,
    colors: &'c Colors,
}

impl<'c> SearchEntry<'c> {
    pub fn new(colors: &'c Colors) -> Self {
        Self {
            text: String::new(),
            colors,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn push(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn pop(&mut self) {
        self.text.pop();
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }
}

impl Widget for &SearchEntry<'_> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let (text, style) = if self.text.is_empty() {
            (DEFAULT_TEXT, Style::default().add_modifier(Modifier::DIM))
        } else {
            (self.text(), Style::default())
        };

        let paragraph = Paragraph::new(Span::styled(text, style)).block(
            Block::default()
                .title(TITLE)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.colors.border))
                .padding(Padding::uniform(1)),
        );

        paragraph.render(area, buf)
    }
}

const TITLE: &str = "Search an emoji";
const DEFAULT_TEXT: &str = "Use arrow keys or type to search";
