use ratatui::prelude::*;
use ratatui::widgets::Widget;
use crate::theme::Theme;
use crate::themes::*;

pub struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: ButtonState,
}

pub enum ButtonState {
    Unselected,
    Selected,
}

impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Button<'a> {
        Button {
            label: label.into(),
            theme: dark_blue::DARK_BLUE_THEME,
            state: ButtonState::Unselected,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Button<'a> {
        self.theme = theme;
        self
    }

    pub fn state(mut self, state: ButtonState) -> Button<'a> {
        self.state = state;
        self
    }

    fn colors(&self) -> (Color, Color) {
        let theme = &self.theme;
        match self.state {
            ButtonState::Unselected => (theme.text, theme.bg),
            ButtonState::Selected => (theme.text, theme.select),
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