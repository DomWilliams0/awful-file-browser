use super::UI;
use awful_files::{FileBrowser, AwfulError};

pub struct Repl {
    fb: FileBrowser,
}

impl Repl {
    pub fn new(fb: FileBrowser) -> Repl {
        Repl { fb: fb }
    }
}

impl UI for Repl {
    fn start(self) -> Result<(), AwfulError> {
        Ok(())
    }
}
