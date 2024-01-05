mod ui;
mod commands;
mod widgets;

use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
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
    pub popup_state: PopupState,
    pub select_index: i8,
}

impl Data {
    pub fn new() -> Self {
        Data {
            current_folder: String::new(),
            popup_state: PopupState::Closed,
            select_index: 0,
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
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                kind,
                code,
                ..
            }) => {
                use KeyCode as key;
                use KeyCode::Char as char;

                match data.popup_state {
                    PopupState::Closed => {
                        match code {
                            key::Esc => {
                                break;
                            }
                            char('c') => {
                                data.popup_state = PopupState::OptionPopup;
                            }
                            char('d') => {
                                data.popup_state = PopupState::TextPopup;
                            }
                            char('r') => {
                                data.popup_state = PopupState::TextPopup;
                            }
                            char('l') => {
                                data.popup_state = PopupState::TextPopup;
                            }
                            _ => {}
                        }
                    }
                    PopupState::OptionPopup => {
                        match code {
                            key::Esc => {
                                data.popup_state = PopupState::Closed;
                            }
                            key::Enter => {
                                // TODO: Enter next state
                            }
                            key::Left => {
                                data.select_index = 0;
                            }
                            key::Right => {
                                data.select_index = 1;
                            }
                            _ => {}
                        }
                    }
                    PopupState::TextPopup => {
                        match code {
                            key::Esc => {
                                data.popup_state = PopupState::Closed;
                            }
                            key::Enter => {
                                // TODO: Enter next state
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        };
        /*
        KeyCode::Backspace => {
            input_text.pop();
        }
        KeyCode::Char(c) => {
            input_text.push(c);
        }
        */
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