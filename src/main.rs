mod ui;

use std::{fs, io, thread};
use std::time::Duration;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::widgets::{Block, Borders};

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

    loop {
        terminal.draw(|f| {
            ui::main_layout(f);
        }).unwrap();
    }
    // load_all_files("C:\\");
}
