use ratatui::prelude::*;
use ratatui::widgets::Widget;
use crate::widgets::colors::*;

pub struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

pub enum State {
    Inactive,
    Hover,
    Active,
}

pub struct Theme {
    text: Color,
    bg: Color,
    hover: Color,
}

pub const DARK_BLUE_BUTTON: Theme = Theme {
    text: COLOR_1_3,
    bg: COLOR_1_1,
    hover: COLOR_1_2,
};

pub const DARK_GREEN_BUTTON: Theme = Theme {
    text: COLOR_2_3,
    bg: COLOR_2_1,
    hover: COLOR_2_2,
};

impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Button<'a> {
        Button {
            label: label.into(),
            theme: DARK_BLUE_BUTTON,
            state: State::Inactive,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Button<'a> {
        self.theme = theme;
        self
    }

    pub fn state(mut self, state: State) -> Button<'a> {
        self.state = state;
        self
    }

    fn colors(&self) -> (Color, Color) {
        let theme = &self.theme;
        match self.state {
            State::Inactive => (theme.text, theme.bg),
            State::Hover => (theme.text, theme.hover),
            State::Active => (theme.text, theme.hover),
        }

    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (text, bg) = self.colors();
        buf.set_style(area, Style::new().bg(bg).fg(text));

        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().bg(bg),
            )
        }
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().bg(bg),
            )
        }
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}