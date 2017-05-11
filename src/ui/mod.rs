// pub use repl;

mod repl;
mod gui;

use awful_files::{FileBrowser, AwfulError};

pub enum UIType {
    Repl,
    Graphical,
}

trait UI {
    fn start(self) -> Result<(), AwfulError>;
}


pub fn start(ui_type: UIType, fb: FileBrowser) -> Result<(), AwfulError> {
    match ui_type {
        UIType::Repl => UI::start(<repl::Repl>::new(fb)),
        _ => Err(AwfulError::NotImplemented)

    }
}
