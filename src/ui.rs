use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};

pub fn main_layout<B: Backend>(f: &mut Frame<B>, path: &str) {
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
    let block_1 = Block::default()
        .title("Current Folder")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(Spans::from(path)).block(block_1);
    f.render_widget(paragraph, chunks[0]);

    // Block to display content inside folder
    let block_2 = Block::default()
        .title("Content")
        .borders(Borders::ALL);
    f.render_widget(block_2, chunks[1]);

    // Block for controlling environment
    let block_3 = Block::default()
        .title("Command Line")
        .borders(Borders::ALL);
    f.render_widget(block_3, chunks[2]);
}