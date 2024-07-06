use ratatui::style::Color;

pub struct Colors {
    pub selected: Color,
    pub unselected: Color,
    pub border: Color,
}

impl Colors {
    pub const LIGHT: Self = Self {
        selected: Color::Green,
        unselected: Color::DarkGray,
        border: Color::DarkGray,
    };

    pub const DARK: Self = Self {
        selected: Color::Green,
        unselected: Color::White,
        border: Color::White,
    };
}
