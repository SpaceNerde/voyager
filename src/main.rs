mod ui;
mod commands;
mod widgets;
mod themes;
mod theme;

use std::{env, fs, io};
use std::thread::sleep;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::{ListItem, ListState};
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
pub struct StateList<'a> {
    state: ListState,
    items: Vec<ListItem<'a>>,
}

impl<'a> StateList<'a> {
    fn new() -> StateList<'a> {
        StateList {
            state: ListState::default(),
            items: vec![],
        }
    }

    fn get_items(self) -> Vec<ListItem<'a>> {
        self.items.to_vec()
    }

    fn add_item(&mut self, item: ListItem<'a>) {
        self.items.push(item);
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1{
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            None => 0,
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
        };
        self.state.select(Some(i));
    }
}

#[derive(Clone)]
pub struct Data<'a> {
    pub popup_state: PopupState,
    pub button_select_index: i8,
    pub list_select_index: i32,
    pub list_max: i32,
    pub input_text: Vec<char>,
    pub task: Task,
    pub items: StateList<'a>,
}

impl<'a> Data<'a> {
    pub fn new() -> Data<'a> {
        Data {
            popup_state: PopupState::Closed,
            // TODO: instead of using an index use an enum
            button_select_index: 0,
            list_select_index: 0,
            list_max: 0,
            input_text: vec![],
            task: Task::Idle,
            items: StateList::new(),
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
        // content logic

        data = load_content(data);

        // main logic

        terminal.draw(|f| {
            ui::main_layout(f, &mut data);
        }).unwrap();

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
                            key::Up => {
                                data.items.previous();
                            }
                            key::Down => {
                                data.items.next();
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
                                data.button_select_index = 0;
                            }
                            key::Right => {
                                data.button_select_index = 1;
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

fn load_content(mut data: Data) -> Data{
    data.items.items.clear();

    for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
        let item = entry.unwrap().file_name();
        if let Some(item_str) = item.to_str() {
            let item_string = item_str.to_string(); // Clone to create an owned String
            let new_item = ListItem::new(item_string);
            data.items.add_item(new_item);
        }
    }

    data
}