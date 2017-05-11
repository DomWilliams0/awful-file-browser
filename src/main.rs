extern crate awful_files;
extern crate clap;

use awful_files::FileBrowser;
use std::error::Error;
use clap::{Arg, App};

struct Config {
    path: String,
    repl: bool,
}

fn main() {
    match parse_args().and_then(|c| run(c)) {
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(_) => {
            std::process::exit(0);
        }
    }
}

fn parse_args() -> Result<Config, Box<Error>> {
    let matches = App::new("awful-file-browser")
        .version("1.0")
        .author("Dom Williams <me@domwillia.ms>")
        .about("Terrible file manager")
        .arg(
          Arg::with_name("path")
          .short("p")
          .takes_value(true)
          .value_name("PATH")
          .default_value(".")
          .help("Specifies the directory to start in")
        )
        .arg(
          Arg::with_name("cmd")
          .short("c")
          .help("Use a command line REPL")
        )
        .arg(
          Arg::with_name("gui")
          .short("g")
          .help("Use a GUI")
        ).get_matches();

        // validation
        let cmd = matches.is_present("cmd");
        let gui = matches.is_present("gui");
        let repl = match (cmd, gui) {
          (true, true) => return Err(From::from("-c and -g cannot both be specified")),
          (true, false) | (false, false) => true,
          (false, true) => false,
        };

    Ok(Config {
        path: String::from(matches.value_of("path").unwrap()),
        repl: repl
    })
}

fn run(conf: Config) -> Result<(), Box<Error>> {
    let fb = FileBrowser::new(conf.path.as_str());

    for file in fb.list_files()?.iter() {
        println!("{} {:?}",
                 if file.file_type.is_dir() { "+" } else { "-" },
                 file.name);
    }
    return Ok(());
}
