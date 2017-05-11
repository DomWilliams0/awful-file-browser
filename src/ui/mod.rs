// pub use repl;

mod repl;
mod gui;

use std::error;
use awful_files::FileBrowser;

pub enum UIType {
    Repl,
    Graphical,
}

trait UI {
    fn start(self) -> Result<(), Box<error::Error>>;
}


pub fn start(ui_type: UIType, fb: FileBrowser) -> Result<(), Box<error::Error>> {
    match ui_type {
        UIType::Repl => UI::start(<repl::Repl>::new(fb)),
        _ => Ok(()), // TODO not implemented error

    }
}
