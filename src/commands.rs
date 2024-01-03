use std::fs::{remove_dir, remove_file, rename};
use crate::Data;

pub fn command_parser(input: &Vec<char>, data: Data) -> Data{
    let input: String = input.iter().cloned().collect();
    let args: Vec<&str> = input.split_whitespace().collect();

    return match args[0] {
        "c" => {
            placeholder();
            data
        },
        "l" => {
            enter_folder(args[1], data)
        },
        "del" => {
            delete_entry(&args, data)
        },
        "r" => {
            rename_entry(&args, data)
        },
        _ => {
            placeholder();
            data
        },
    }
}

fn enter_folder(path: &str, mut data: Data) -> Data{
    data.current_folder = path.parse().unwrap();
    data
}

fn delete_entry(args: &Vec<&str>, data: Data) -> Data {
    return match args[1] {
        "d" => {
            remove_dir(data.current_folder.clone() + "/" + args[2]).unwrap();
            data
        },
        "f" => {
            remove_file(data.current_folder.clone() + "/" + args[2]).unwrap();
            data
        },
        _ => {
            data
        }
    }

}

fn rename_entry(args: &Vec<&str>, mut data: Data) -> Data {
    rename(data.current_folder.clone() + "/" + args[1], data.current_folder.clone() + "/" + args[2]).unwrap();
    data
}

fn placeholder() {}