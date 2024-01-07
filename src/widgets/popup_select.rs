use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::Widget;
use crate::theme::Theme;
use crate::themes::dark_blue::DARK_BLUE_THEME;

pub struct PopupSelect {
    theme: Theme
}

impl PopupSelect {
    pub fn new() -> PopupSelect {
        PopupSelect {
            theme: DARK_BLUE_THEME,
        }
    }
    pub fn theme(mut self, theme: Theme) -> PopupSelect {
        self.theme = theme;
        self
    }
    fn colors(&self) -> (Color, Color) {
        let theme = &self.theme;
        (theme.text, theme.bg)
    }
}

impl Widget for PopupSelect {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (_text, bg) = self.colors();
        buf.set_style(area, Style::new().bg(bg));
    }
}