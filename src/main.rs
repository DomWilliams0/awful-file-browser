extern crate awful_files;

use awful_files::FileBrowser;
use std::io;
use std::env;

fn main() {
    match run(parse_path()) {
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(_) => {
            std::process::exit(0);
        }
    }
}

fn parse_path() -> String {
    env::args().nth(1).unwrap_or(String::from("."))
}

fn run(path: String) -> Result<(), io::Error> {
    let fb = FileBrowser::new(path.as_str());

    for file in fb.list_files()?.iter() {
        println!("{} {:?}",
                 if file.file_type.is_dir() { "+" } else { "-" },
                 file.name);
    }
    return Ok(());


}
