use std::{env, fs};
use std::fs::{DirBuilder, File};
use std::path::{Path, PathBuf};
use crate::{Data, EntryKind, PopupState, Task};

pub fn button_select(mut data: Data) -> Data {
    match data.button_select_index {
        0 => {
            data.popup_state = PopupState::TextPopup;
            match &mut data.task {
                Task::Create(entry_type) => {
                    *entry_type = EntryKind::File
                }
                _ => {}
            }
        }
        1 => {
            data.popup_state = PopupState::TextPopup;
            match &mut data.task {
                Task::Create(entry_type) => {
                    *entry_type = EntryKind::Directory
                }
                _ => {}
            }
            data.button_select_index = 0;
        }
        _ => {
            eprintln!("YOU BROKE IT HOW THE FUCK DID YOU BREAK THIS?!?!?!?!??!");
        }
    }
    data
}

pub fn parse_command(data: Data){
    match data.task {
        Task::Idle => {}
        Task::Delete => {

        }
        Task::Create(ref entry_type) => {
            match entry_type {
                EntryKind::Pending => {}
                EntryKind::File => {
                    match File::create(get_path(data.clone())) {
                        Ok(_) => {}
                        Err(err) => eprintln!("Error creating file: {:?}", err),
                    }
                }
                EntryKind::Directory => {
                    match DirBuilder::new().recursive(true).create(get_path(data.clone())) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
            }
        }
        Task::Load => {
            env::set_current_dir(format!("{}", data.input_text.iter().cloned().collect::<String>())).unwrap();
        }
        Task::Rename(from, to) => {
            let to = data.input_text.iter().cloned().collect::<String>();
            fs::rename(from, to).unwrap();
        }
    }
}

fn get_path(data: Data) -> PathBuf {
    let current_dir = env::current_dir().unwrap();

    let input_text_str: String = data.input_text.iter().cloned().collect();
    let path_buf: Vec<PathBuf> = [current_dir.as_path(), Path::new(&input_text_str)]
        .iter()
        .map(|path| PathBuf::from(path))
        .collect();

    path_buf.iter().fold(PathBuf::new(), |acc, p| acc.join(p))
}