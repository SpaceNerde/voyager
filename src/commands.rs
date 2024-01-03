use crate::Data;

pub fn command_parser(input: &Vec<char>, mut data: Data) -> Data{
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
            placeholder();
            data
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

fn placeholder() {}