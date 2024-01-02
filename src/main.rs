mod ui;

use std::{fs, io};
use crossterm::event::{EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
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

    loop {
        terminal.draw(|f| {
            ui::main_layout(f, "c");
        }).unwrap();
    }
    // load_all_files("C:\\");
}
