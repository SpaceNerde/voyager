mod ui;
mod commands;

use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind};
use crossterm::event::Event::Key;
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub struct Data {
    pub current_folder: String
}

impl Data {
    pub fn new() -> Self {
        Data {
            current_folder: String::new(),
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
                        break;
                    }
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

