use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Padding, Row, StatefulWidget, Table, TableState, Widget},
};
use regex::RegexBuilder;

use crate::{
    colors::Colors,
    emoji::{Emoji, EMOJIS},
};

pub struct SelectionView {
    state: TableState,
    colors: Colors,
}

impl SelectionView {
    pub fn new(colors: Colors) -> Self {
        Self {
            state: TableState::default().with_selected((!EMOJIS.is_empty()).then_some(0)),
            colors,
        }
    }

    pub fn filtered_view(&mut self, search_text: &str) -> FilteredView {
        let pattern = RegexBuilder::new(search_text)
            .case_insensitive(true)
            .build()
            .expect("Invalid characters in search text");

        let emojis: Box<[&Emoji]> = EMOJIS
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
    colors: Colors,
}

impl FilteredView<'_> {
    pub fn selected(&self) -> Option<&Emoji> {
        self.state
            .selected()
            .and_then(|idx| self.emojis.get(idx))
            .copied()
    }

    pub fn move_up(&mut self) {
        let Some(idx) = self.state.selected_mut() else {
            return;
        };

        if *idx > 0 {
            *idx -= 1;
        } else if *idx == 0 {
            // At this point emojis is guaranteed to be not empty
            *idx = self.emojis.len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        let Some(idx) = self.state.selected_mut() else {
            return;
        };

        // At this point emojis is guaranteed to be not empty
        if *idx == self.emojis.len() - 1 {
            *idx = 0;
        } else {
            *idx += 1;
        }
    }
}

impl Widget for &mut FilteredView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self
            .emojis
            .iter()
            .map(|emoji| Row::new([emoji.emoji, emoji.code, emoji.description]));

        let widths = [
            Constraint::Percentage(3),
            Constraint::Percentage(12),
            Constraint::Percentage(85),
        ];

        let table = Table::new(rows, widths)
            .block(
                Block::default()
                    .title(BLOCK_TITLE)
                    .borders(Borders::ALL)
                    .padding(Padding::new(1, 1, 1, 0)),
            )
            .style(Style::default().fg(self.colors.unselected))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(self.colors.selected),
            )
            .highlight_symbol(HIGHLIGHT_SYMBOL)
            .column_spacing(2);

        StatefulWidget::render(table, area, buf, self.state);
    }
}

const BLOCK_TITLE: &str = "Select an emoji";
const HIGHLIGHT_SYMBOL: &str = "> ";
