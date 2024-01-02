use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};

pub fn main_layout<B: Backend>(f: &mut Frame<B>) {
    // define areas
    let chunks_left = Layout::default()
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
    let block_1 = Block::default()
        .title("Current Folder")
        .borders(Borders::ALL);

    let block_2 = Block::default()
        .title("Content")
        .borders(Borders::ALL);

    let block_3 = Block::default()
        .title("Command Line")
        .borders(Borders::ALL);

    f.render_widget(block_1, chunks_left[0]);
    f.render_widget(block_2, chunks_left[1]);
    f.render_widget(block_3, chunks_left[2]);
}