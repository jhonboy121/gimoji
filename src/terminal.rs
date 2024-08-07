use anyhow::Context;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
};
use std::io::{self, Stdout};

use crate::{
    colors::Colors,
    search_entry::SearchEntry,
    selection_view::{FilteredView, SelectionView},
};

pub struct Terminal {
    term: ratatui::Terminal<CrosstermBackend<Stdout>>,
    search_entry: SearchEntry,
    selection_view: SelectionView,
}

#[derive(Default)]
pub enum EventResponse {
    #[default]
    Noop,
    EmojiSelected(&'static str),
    Exit,
}

impl Terminal {
    pub fn new(colors: Colors) -> anyhow::Result<Self> {
        terminal::enable_raw_mode().context("Failed to enable raw mode")?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
        let backend = CrosstermBackend::new(stdout);
        let search_entry = SearchEntry::new(colors);
        let selection_view = SelectionView::new(colors);
        let term = ratatui::Terminal::new(backend).context("Failed to create terminal instance")?;
        Ok(Self {
            term,
            search_entry,
            selection_view,
        })
    }

    pub fn render_ui(&mut self) -> anyhow::Result<EventResponse> {
        let mut filtered_view = self.selection_view.filtered_view(self.search_entry.text());

        self.term
            .draw(|f| {
                let chunks = Layout::default()
                    .constraints([Constraint::Min(5), Constraint::Percentage(100)].as_ref())
                    .margin(1)
                    .split(f.size());

                // The search entry goes at the top.
                f.render_widget(&self.search_entry, chunks[0]);

                // The emoji list.
                f.render_widget(&mut filtered_view, chunks[1]);
            })
            .context("Failed to render widgets")?;

        event::read()
            .map(|event| match event {
                Event::Key(key_event) => {
                    Self::handle_key_event(key_event, &mut self.search_entry, &mut filtered_view)
                }
                _ => EventResponse::Noop,
            })
            .context("Failed to read UI event")
    }

    fn handle_key_event(
        event: KeyEvent,
        search_entry: &mut SearchEntry,
        filtered_view: &mut FilteredView,
    ) -> EventResponse {
        match event.code {
            KeyCode::Enter => filtered_view
                .selected()
                .map(|emoji| emoji.emoji)
                .map(EventResponse::EmojiSelected)
                .unwrap_or_default(),
            KeyCode::Esc => {
                if search_entry.text().is_empty() {
                    EventResponse::Exit
                } else {
                    search_entry.clear();
                    EventResponse::Noop
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                filtered_view.move_down();
                EventResponse::Noop
            }
            KeyCode::Up | KeyCode::BackTab => {
                filtered_view.move_up();
                EventResponse::Noop
            }
            KeyCode::Char(ch) => {
                if ch == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                    EventResponse::Exit
                } else {
                    search_entry.push(ch);
                    EventResponse::Noop
                }
            }
            KeyCode::Backspace => {
                search_entry.pop();
                EventResponse::Noop
            }
            _ => EventResponse::Noop,
        }
    }

    pub fn reset(&mut self) -> anyhow::Result<()> {
        terminal::disable_raw_mode().context("Failed to disable raw mode")?;
        execute!(self.term.backend_mut(), LeaveAlternateScreen)
            .context("Failed to leave alternate screen")?;
        self.term
            .show_cursor()
            .context("Failed to show terminal cursor")
    }
}
