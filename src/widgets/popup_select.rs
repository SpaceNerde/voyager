use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::Widget;
use crate::widgets::button::{Button, ButtonState};
use crate::widgets::colors::*;

pub struct PopupSelect {
    theme: PopupSelectTheme
}

// TODO: DONT DUCKING HARDCODE THEMES!!!!!!!!! ( every time once is ok :D )

pub struct PopupSelectTheme {
    text: Color,
    bg: Color,
}

pub const DARK_BLUE_POPUP_SELECT: PopupSelectTheme = PopupSelectTheme {
    text: COLOR_1_3,
    bg: COLOR_1_1,
};

pub const DARK_GREEN_POPUP_SELECT: PopupSelectTheme = PopupSelectTheme {
    text: COLOR_2_3,
    bg: COLOR_2_1,
};

impl PopupSelect {
    pub fn new() -> PopupSelect {
        PopupSelect {
            theme: DARK_BLUE_POPUP_SELECT,
        }
    }
    pub fn theme(mut self, theme: PopupSelectTheme) -> PopupSelect {
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
        let (text, bg) = self.colors();
        buf.set_style(area, Style::new().bg(bg));
    }
}