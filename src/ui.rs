use std::fs;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::Data;

pub fn main_layout<B: Backend>(f: &mut Frame<B>, path: &str, input_text: &Vec<char>, data: &Data) {
    // define areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ].as_ref()
        )
        .split(f.size());

    // config widgets
    // Block to display what folder you are in
    let block = Block::default()
        .title("Current Folder")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(Spans::from(path)).block(block);
    f.render_widget(paragraph, chunks[0]);

    // Block to display content inside folder
    let block = Block::default()
        .title("Content")
        .borders(Borders::ALL);
    let list = List::new(load_folder(&data)).block(block);
    f.render_widget(list, chunks[1]);

    // Input field
    let block = Block::default()
        .title("Command Line")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(input_text.iter().cloned().collect::<String>()).block(block);
    f.render_widget(paragraph, chunks[2]);
}

fn load_folder(data: &Data) -> Vec<ListItem> {
    let mut list_items: Vec<ListItem> = Vec::new();
    for entry in fs::read_dir(&data.current_folder).unwrap() {
        let item = entry.unwrap();
        let file_name: Option<String> = item.file_name().to_str().map(|s| s.to_string());
        list_items.push(ListItem::new(file_name.unwrap()));
    }
    list_items
}