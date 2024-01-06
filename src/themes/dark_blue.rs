use ratatui::prelude::Color;
use crate::theme::Theme;

pub const DARK_BLUE_THEME: Theme = Theme {
    bg: Color::Rgb(22, 26, 48),
    text: Color::Rgb(182, 187, 196),
    select: Color::Rgb(49, 48, 77),
};