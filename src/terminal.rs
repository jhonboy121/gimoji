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
    emoji::Emoji,
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
    EmojiSelected(String),
    Exit,
}

impl Terminal {
    pub fn new(colors: Colors) -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let search_entry = SearchEntry::new(colors);
        let selection_view = SelectionView::new(colors);
        let term = ratatui::Terminal::new(backend)?;
        Ok(Self {
            term,
            search_entry,
            selection_view,
        })
    }

    pub fn render_ui(&mut self) -> io::Result<EventResponse> {
        let mut filtered_view = self.selection_view.filtered_view(self.search_entry.text());

        self.term.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(5), Constraint::Percentage(100)].as_ref())
                .margin(1)
                .split(f.size());

            // The search entry goes at the top.
            f.render_widget(&self.search_entry, chunks[0]);

            // The emoji list.
            f.render_widget(&mut filtered_view, chunks[1]);
        })?;

        event::read().map(|event| match event {
            Event::Key(key_event) => {
                Self::handle_key_event(key_event, &mut self.search_entry, &mut filtered_view)
            }
            _ => EventResponse::Noop,
        })
    }

    fn handle_key_event(
        event: KeyEvent,
        search_entry: &mut SearchEntry,
        filtered_view: &mut FilteredView,
    ) -> EventResponse {
        match event.code {
            KeyCode::Enter => filtered_view
                .selected()
                .map(Emoji::emoji)
                .map(str::to_owned)
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
            KeyCode::Down => {
                filtered_view.move_down();
                EventResponse::Noop
            }
            KeyCode::Up => {
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

    pub fn reset(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(self.term.backend_mut(), LeaveAlternateScreen)?;
        self.term.show_cursor()
    }
}
