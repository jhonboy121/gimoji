use crate::colors::Colors;
use ratatui::{
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub struct SearchEntry {
    buf: String,
    colors: Colors,
}

impl SearchEntry {
    pub fn new(colors: Colors) -> Self {
        Self {
            buf: String::new(),
            colors,
        }
    }

    pub fn text(&self) -> &str {
        &self.buf
    }

    pub fn push(&mut self, c: char) {
        self.buf.push(c);
    }

    pub fn pop(&mut self) {
        self.buf.pop();
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }
}

impl Widget for &SearchEntry {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let (text, style) = if self.buf.is_empty() {
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
