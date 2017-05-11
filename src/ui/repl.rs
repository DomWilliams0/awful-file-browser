use std::io::{stdin, BufRead};
use super::UI;
use awful_files::{FileBrowser, AwfulError};

pub struct Repl {
    fb: FileBrowser,
    active: bool,
}

impl Repl {
    pub fn new(fb: FileBrowser) -> Repl {
        Repl {
            fb: fb,
            active: true,
        }
    }

    fn handle_command(&self, cmd: &str, args: Option<&str>) {
      println!("Command {:?} with {}", cmd, if let Some(arg) = args {
        format!("argument '{}'", arg)
      } else {
        String::from("no argument")
      });
    }
}

impl UI for Repl {
    fn start(self) -> Result<(), AwfulError> {

        let stdin = stdin();
        for line in stdin.lock().lines() {
            if !self.active {
                break;
            }

            let line = line.unwrap();
            let mut split = line.splitn(2, ' ');
            if let Some(cmd) = split.next() {
                let args = split.next();

                self.handle_command(cmd, args);
                continue;
            }

            println!("Unknown command");


        }

        println!("goodbye");
        Ok(())
    }

}
