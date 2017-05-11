use std::io::{stdin, BufRead};
use super::UI;
use awful_files::{FileBrowser, AwfulError};

struct Command {
  pub needs_arg: bool,
  pub handler: fn(&mut Repl, Option<&str>)
}

  static HANDLERS: &'static [(&'static str, Command)] = &[
    ("test", Command {needs_arg: false, handler: Repl::handler_test}),
    ];


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

    fn get_handler(cmd: &str) -> Option<&Command> {
       HANDLERS.iter()
       .find(|ref x| x.0 == cmd)
       .map(|tup| &tup.1)
    }


    fn handle_command(&mut self, cmd: &str, args: Option<&str>) -> Result<(), AwfulError> {

      let handler = Repl::get_handler(cmd);
      match handler {
        None => Err(AwfulError::Args("Unknown command")),
        Some(cmd) => {

          match (cmd.needs_arg, args.is_some()) {
            (true, false) => return Err(AwfulError::Args("This command requires an argument")),
            (false, true) => return Err(AwfulError::Args("This command does not take any arguments")),
            _ => ()
          }

          (cmd.handler)(self, args);
          Ok(())
        }
      }
    }

    fn handler_test(&mut self, _: Option<&str>) {
      println!("Testing!");
    }

}

impl UI for Repl {
    fn start(&mut self) -> Result<(), AwfulError> {

        let stdin = stdin();
        for line in stdin.lock().lines() {
            if !self.active {
                break;
            }

            let line = line.unwrap();
            let mut split = line.splitn(2, ' ');
            if let Some(cmd) = split.next() {
                let args = split.next();

                if let Err(err) = self.handle_command(cmd, args) {
                  println!("{}", err);
                }
                continue;
            }

            println!("Unknown command");


        }

        println!("Goodbye!");
        Ok(())
    }

}
