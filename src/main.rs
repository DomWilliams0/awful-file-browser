extern crate awful_files;
extern crate clap;
mod ui;

use awful_files::{FileBrowser, AwfulError};
use clap::{Arg, App};

struct Config {
    path: String,
    ui: ui::UIType,
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

fn parse_args() -> Result<Config, AwfulError> {
    let matches = App::new("awful-file-browser")
        .version("1.0")
        .author("Dom Williams <me@domwillia.ms>")
        .about("Terrible file manager")
        .arg(Arg::with_name("path")
                 .short("p")
                 .takes_value(true)
                 .value_name("PATH")
                 .default_value(".")
                 .help("Specifies the directory to start in"))
        .arg(Arg::with_name("cmd")
                 .short("c")
                 .help("Use a command line REPL"))
        .arg(Arg::with_name("gui").short("g").help("Use a GUI"))
        .get_matches();

    // validation
    let cmd = matches.is_present("cmd");
    let gui = matches.is_present("gui");
    let ui = match (cmd, gui) {
        (true, true) => return Err(AwfulError::Args("-c and -g cannot both be specified")),
        (_, false) => ui::UIType::Repl,
        (_, true) => ui::UIType::Graphical,
    };

    Ok(Config {
           path: String::from(matches.value_of("path").unwrap()),
           ui: ui,
       })
}

fn run(conf: Config) -> Result<(), AwfulError> {
    let fb = FileBrowser::new(conf.path.as_str());

    ui::start(conf.ui, fb)
}
