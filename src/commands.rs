use std::{env, fs};
use std::fs::{DirBuilder, File, remove_dir, remove_file, rename};
use std::path::{Path, PathBuf};
use crate::{Data, EntryKind, PopupState, Task};


// how to make the command parser?
// using regex to extract the arguments [command] [args]
// or maybe
// no more console and make like a pop up window
// press c for creating a entity and then like choose between file and folder but with popup windows?
// choose between and then choose a name

// errors will pop up

// new code

pub fn button_select(mut data: Data) -> Data {
    match data.select_index {
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
            data.select_index = 0;
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
        Task::Rename => {
            // TODO
        }
    }
}

fn get_path(data: Data) -> PathBuf{
    let current_dir = env::current_dir().unwrap();

    let input_text_str: String = data.input_text.iter().cloned().collect();
    let path_buf: Vec<PathBuf> = [current_dir.as_path(), Path::new(&input_text_str)]
        .iter()
        .map(|path| PathBuf::from(path))
        .collect();

    path_buf.iter().fold(PathBuf::new(), |acc, p| acc.join(p))
}

// old code
/*
pub fn command_parser(input: &Vec<char>, data: Data) -> Data {
    let input: String = input.iter().cloned().collect();
    let args: Vec<&str> = input.split_whitespace().collect();

    // TODO make this shit better and not hardcoded
    return match args.get(0){
        Some(arg) => {
            match arg.trim() {
                "c" => {
                    if args.len() == 3 {
                        create_entry(&args, data)
                    } else {
                        data
                    }
                },
                "l" => {
                    if args.len() == 2 {
                        enter_folder(args[1], data)
                    } else {
                        data
                    }
                },
                "del" => {
                    if args.len() == 3 {
                        delete_entry(&args, data)
                    }  else {
                        data
                    }
                },
                "r" => {
                    if args.len() == 3 {
                        rename_entry(&args, data)
                    } else {
                        data
                    }
                },
                _ => {
                    placeholder();
                    data
                },
            }
        }

        None => {
            data
        },
    };
}

fn enter_folder(path: &str, mut data: Data) -> Data{
    match Path::exists(path.as_ref()) {
        true => {
            data.current_folder = path.parse().unwrap();
        }
        false => {
            // TODO: path does not exsist
            return data;
        }
    }
    data
}

fn delete_entry(args: &Vec<&str>, data: Data) -> Data {
    let p = data.current_folder.clone() + "/" + args[2];

    return match args[1] {
        "d" => {
            if Path::exists(p.as_ref()) {
                remove_dir(p).unwrap();
            }
            data
        },
        "f" => {
            if Path::exists(p.as_ref()) {
                remove_file(p).unwrap();
            }
            data
        },
        _ => {
            data

        }
    }
}

fn rename_entry(args: &Vec<&str>, data: Data) -> Data {
    rename(data.current_folder.clone() + "/" + args[1], data.current_folder.clone() + "/" + args[2]).unwrap();
    data
}

fn create_entry(args: &Vec<&str>, data: Data) -> Data {
    let p = data.current_folder.clone() + "/" + args[2].as_ref();

    return match args[1] {
        "d" => {
            DirBuilder::new()
                .recursive(true)
                .create(p).unwrap();
            data
        },
        "f" => {
            File::create(p).unwrap();
            data
        },
        _ => {
            // TODO: missing args
            data
        }
    }
}

fn placeholder() {}
*/
