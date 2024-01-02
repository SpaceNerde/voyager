mod ui;

use std::{fs, io};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind};
use crossterm::event::Event::Key;
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn load_all_files(path: &str) {
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        println!("{:?}", dir.path());
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut input_text:Vec<char> = Vec::new();

    loop {
        terminal.draw(|f| {
            ui::main_layout(f, "c", &input_text);
        }).unwrap();

        if let Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        break;
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
    }
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();

    terminal.clear().unwrap();
    // load_all_files("C:\\");
}

