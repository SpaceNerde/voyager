mod ui;
mod commands;
mod widgets;

use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind};
use crossterm::event::Event::Key;
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub enum PopupState {
    Closed,
    OptionPopup,
    TextPopup
}

pub struct Data {
    pub current_folder: String,
    pub popup_state: PopupState
}

impl Data {
    pub fn new() -> Self {
        Data {
            current_folder: String::new(),
            popup_state: PopupState::Closed,
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut data = Data::new();
    let mut input_text:Vec<char> = Vec::new();
    data.current_folder = "C://".to_string();
    loop {
        if let Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        match data.popup_state {
                            PopupState::Closed => {
                                break;
                            }
                            PopupState::OptionPopup => {
                                data.popup_state = PopupState::Closed;
                            }
                            PopupState::TextPopup => {
                                data.popup_state = PopupState::Closed;
                            }
                        }
                    }
                    /*
                    KeyCode::Enter => {
                        data = commands::command_parser(&input_text, data);
                        input_text.clear();
                    }
                    KeyCode::Backspace => {
                        input_text.pop();
                    }
                    KeyCode::Char(c) => {
                        input_text.push(c);
                    }
                    */
                    KeyCode::Char('c') => {
                        data.popup_state = PopupState::OptionPopup;
                    },
                    KeyCode::Char('d') => {
                        data.popup_state = PopupState::TextPopup;
                    },
                    KeyCode::Char('r') => {
                        data.popup_state = PopupState::TextPopup;
                    },
                    _ => {}
                }
            }
        }

        terminal.draw(|f| {
            ui::main_layout(f, &input_text, &data);
        }).unwrap();
    }
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();

    terminal.clear().unwrap();
}

