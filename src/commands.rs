use std::fs::{DirBuilder, File, remove_dir, remove_file, rename};
use std::path::Path;
use crate::Data;
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