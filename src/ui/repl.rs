use std::cmp;
use std::fs;
use std::io::{stdin, BufRead};
use super::UI;
use awful_files::{FileBrowser, AwfulError};

struct Command {
    pub needs_arg: bool,
    pub handler: fn(&mut Repl, Option<&str>) -> Result<(), AwfulError>,
}

static HANDLERS: &'static [(&'static str, Command)] = &[("quit",
                                                         Command {
                                                             needs_arg: false,
                                                             handler: Repl::handler_quit,
                                                         }),
                                                        ("ls",
                                                         Command {
                                                             needs_arg: false,
                                                             handler: Repl::handler_list,
                                                         }),
                                                        ("cd",
                                                         Command {
                                                             needs_arg: true,
                                                             handler: Repl::handler_cd,
                                                         })];


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
        HANDLERS.iter().find(|ref x| x.0 == cmd).map(|tup| &tup.1)
    }


    fn handle_command(&mut self, cmd: &str, args: Option<&str>) -> Result<(), AwfulError> {

        let handler = Repl::get_handler(cmd);
        match handler {
            None => Err(AwfulError::Args("Unknown command")),
            Some(cmd) => {

                match (cmd.needs_arg, args.is_some()) {
                    (true, false) => {
                        return Err(AwfulError::Args("This command requires an argument"))
                    }
                    (false, true) => {
                        return Err(AwfulError::Args("This command does not take any arguments"))
                    }
                    _ => (),
                }

                (cmd.handler)(self, args)?;
                Ok(())
            }
        }
    }

    fn handler_quit(&mut self, _: Option<&str>) -> Result<(), AwfulError> {
        self.active = false;
        Ok(())
    }

    fn handler_list(&mut self, _: Option<&str>) -> Result<(), AwfulError> {
        let mut files = self.fb.list_files()?;
        let max_len = cmp::max(10, files.iter().map(|x| x.name().len()).max().unwrap_or(0));


        println!("Listing {:?}", fs::canonicalize(self.fb.path()).unwrap());
        for _ in 0..max_len {
            print!("-");
        }
        println!();

        // sort dirs first
        files.sort_by_key(|x| !x.file_type().is_dir());

        for file in files.iter() {
            println!("{} {:?}",
                     if file.file_type().is_dir() { "+" } else { "-" },
                     file.name());
        }

        Ok(())
    }

    fn handler_cd(&mut self, arg: Option<&str>) -> Result<(), AwfulError> {
        self.fb.cd(arg.unwrap())?;
        Ok(())
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
