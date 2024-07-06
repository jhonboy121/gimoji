use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Padding, Row, StatefulWidget, Table, TableState, Widget},
};
use regex::RegexBuilder;

use crate::colors::Colors;
use crate::emoji::Emoji;

pub struct SelectionView<'e, 'c> {
    emojis: &'e [Emoji],
    state: TableState,
    colors: &'c Colors,
}

impl<'e, 'c> SelectionView<'e, 'c> {
    pub fn new(emojis: &'e [Emoji], colors: &'c Colors) -> Self {
        Self {
            emojis,
            state: TableState::default().with_selected((!emojis.is_empty()).then_some(0)),
            colors,
        }
    }

    pub fn filtered_view(&mut self, search_text: &str) -> FilteredView {
        let pattern = RegexBuilder::new(search_text)
            .case_insensitive(true)
            .build()
            .expect("Invalid characters in search text");

        let emojis: Box<[&Emoji]> = self
            .emojis
            .iter()
            .filter(|emoji| emoji.contains(&pattern))
            .collect();

        match self.state.selected() {
            Some(idx) => {
                // Reset the selection if the list goes shorter than the selected index.
                if emojis.is_empty() {
                    self.state.select(None);
                } else if idx >= emojis.len() {
                    self.state.select(Some(0));
                }
            }
            None => {
                if !emojis.is_empty() {
                    self.state.select(Some(0))
                }
            }
        }

        FilteredView {
            emojis,
            state: &mut self.state,
            colors: self.colors,
        }
    }
}

pub struct FilteredView<'s> {
    emojis: Box<[&'s Emoji]>,
    state: &'s mut TableState,
    colors: &'s Colors,
}

impl FilteredView<'_> {
    pub fn selected(&self) -> Option<&Emoji> {
        self.emojis.get(self.state.selected().unwrap()).copied()
    }

    pub fn move_up(&mut self) {
        let i = self.state.selected().unwrap();
        let i = if i == 0 { self.emojis.len() - 1 } else { i - 1 };
        self.state.select(Some(i));
    }

    pub fn move_down(&mut self) {
        let i = self.state.selected().unwrap();
        let i = if i == self.emojis.len() - 1 { 0 } else { i + 1 };
        self.state.select(Some(i));
    }
}

impl Widget for &mut FilteredView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let emojis = self
            .emojis
            .iter()
            .map(|emoji| Row::new(vec![emoji.emoji(), emoji.code(), emoji.description()]));
        let table = Table::new(
            emojis,
            [
                Constraint::Percentage(3),
                Constraint::Percentage(12),
                Constraint::Percentage(85),
            ],
        )
        .block(
            Block::default()
                .title("Select an emoji")
                .borders(Borders::ALL)
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 1,
                    bottom: 0,
                }),
        )
        .style(Style::default().fg(self.colors.unselected))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(self.colors.selected),
        )
        .highlight_symbol("❯ ")
        .column_spacing(2);
        StatefulWidget::render(table, area, buf, self.state);
    }
}
