use ratatui::prelude::*;
use crate::widgets::colors::*;

struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

enum State {
    Inactive,
    Hover,
    Active,
}

struct Theme {
    text: Color,
    bg: Color,
    hover: Color,
}

const DARK_BLUE_BUTTON: Theme = Theme {
    text: COLOR_1_3,
    bg: COLOR_1_1,
    hover: COLOR_1_2,
};

const DARK_GREEN_BUTTON: Theme = Theme {
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
}