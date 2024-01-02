use std::fs;

fn load_all_files(path: &str) {
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        println!("{:?}", dir.path());
    }
}

fn main() {
    println!("Starting up Voyager...");

    load_all_files("C:\\");
}
