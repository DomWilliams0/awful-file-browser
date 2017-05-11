use super::UI;
use std::error;
use awful_files::FileBrowser;

pub struct Repl {
    fb: FileBrowser,
}

impl Repl {
    pub fn new(fb: FileBrowser) -> Repl {
        Repl { fb: fb }
    }
}

impl UI for Repl {
    fn start(self) -> Result<(), Box<error::Error>> {
        Ok(())
    }
}
