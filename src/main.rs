mod ui;
mod commands;
mod widgets;
mod themes;
mod theme;

use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::commands::{button_select, parse_command};
use crate::EntryKind::Pending;
use crate::Task::{Create, Delete, Load, Rename};
#[derive(Clone)]
pub enum PopupState {
    Closed,
    OptionPopup,
    TextPopup
}
#[derive(Clone)]
pub enum EntryKind {
    Pending,
    File,
    Directory,
}
#[derive(Clone)]
pub enum Task {
    Idle,
    Delete,
    Create(EntryKind),
    Load,
    Rename,
}
#[derive(Clone)]
pub struct Data {
    pub popup_state: PopupState,
    pub select_index: i8,
    pub input_text: Vec<char>,
    pub task: Task,
}

impl Data {
    pub fn new() -> Self {
        Data {
            popup_state: PopupState::Closed,
            // TODO: instead of using an index use an enum
            select_index: 0,
            input_text: vec![],
            task: Task::Idle,
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

    loop {
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                kind,
                code,
                ..
            }) => {
                use KeyCode as key;
                use KeyCode::Char as char;

                if kind != KeyEventKind::Press {
                    continue;
                }

                match data.popup_state {
                    PopupState::Closed => {
                        match code {
                            key::Esc => {
                                break;
                            }
                            char('c') => {
                                data.popup_state = PopupState::OptionPopup;
                                data.task = Create(Pending);
                            }
                            char('d') => {
                                data.popup_state = PopupState::TextPopup;
                                data.task = Delete;
                            }
                            char('r') => {
                                data.task = Rename;
                            }
                            char('l') => {
                                data.popup_state = PopupState::TextPopup;
                                data.task = Load;
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
                                data = button_select(data);
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
                                parse_command(data.clone());
                                data.input_text.clear();
                                data.popup_state = PopupState::Closed;
                            }
                            key::Backspace => {
                                data.input_text.pop();
                            }
                            char(c) => {
                                data.input_text.push(c);
                            }
                            // TODO
                            key::Left => {}
                            key::Right => {}
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
            ui::main_layout(f, &data);
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
