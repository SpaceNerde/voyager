use std::fs;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Line, Rect};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget};
use crate::{Data, PopupState};
use crate::widgets::button::*;
use crate::widgets::popup_select::*;

pub fn main_layout(f: &mut Frame,input_text: &Vec<char>, data: &Data) {
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
    let current_folder = data.current_folder.clone();
    let paragraph = Paragraph::new(Span::from(current_folder.as_str())).block(block);
    f.render_widget(paragraph, chunks[0]);

    // Block to display content inside folder
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ].as_ref()
        )
        .split(chunks[1]);

    let content_block= Block::default()
        .title("Content")
        .borders(Borders::ALL);
    let list = List::new(load_folder(&data)).block(content_block);
    f.render_widget(list, content_chunks[0]);

    let help_block= Block::default()
        .title("Commands")
        .borders(Borders::ALL);
    let list = List::new([
        ListItem::new(Span::from("d - Directory / f - File")),
        ListItem::new(Span::from("")),
        ListItem::new(Span::from("l [path]: Load folder")),
        ListItem::new(Span::from("r [old name] [new name]: Rename Folder or File")),
        ListItem::new(Span::from("c [f/d] [name]: Creates Folder or File ")),
        ListItem::new(Span::from("del [f/d] [name]: Deletes Folder or File")),
    ]).block(help_block);
    f.render_widget(list, content_chunks[1]);

    match data.popup_state {
        PopupState::Closed => {}
        PopupState::OptionPopup => {
            let area = centered_rect(60, 20, f.size());
            let split_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref()
                ).split(area);

            // TODO: Add margin and make buttons appendable with popup widget without this mess

            let popup = PopupSelect::new().theme(DARK_BLUE_POPUP_SELECT);

            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(popup, area);

            let mut button_1 = Button::new("Test 1").theme(DARK_BLUE_BUTTON);
            let mut button_2 = Button::new("Test 2").theme(DARK_BLUE_BUTTON);

            match data.select_index {
                0 => {
                    button_2 = button_2.state(ButtonState::Unselected);
                    button_1 = button_1.state(ButtonState::Selected);
                }
                1 => {
                    button_1 = button_1.state(ButtonState::Unselected);
                    button_2 = button_2.state(ButtonState::Selected);
                }
                _ => {
                    eprintln!("YOU BROKE IT HOW THE FUCK DID YOU BREAK THIS?!?!?!?!??!")
                }
            }


            f.render_widget(button_1, split_area[0]);
            f.render_widget(button_2, split_area[1]);
        }
        PopupState::TextPopup => {
            let block = Block::default().title("Popup").borders(Borders::ALL);
            let area = centered_rect(60, 20, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(block, area);
        }
    }

    // Input field
    let block= Block::default()
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}