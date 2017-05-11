// pub use repl;

mod repl;
mod gui;

use awful_files::{FileBrowser, AwfulError};

pub enum UIType {
    Repl,
    Graphical,
}

trait UI {
    fn start(&mut self) -> Result<(), AwfulError>;
}


pub fn start(ui_type: UIType, fb: FileBrowser) -> Result<(), AwfulError> {
    let mut ui = match ui_type {
        UIType::Repl => repl::Repl::new(fb),
        _ => return Err(AwfulError::NotImplemented),
    };

    ui.start()
}
