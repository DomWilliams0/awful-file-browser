use std::path::PathBuf;
use std::io;
use std::ffi;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum AwfulError {
    Args(&'static str),
    NotImplemented,
    Io(std::io::Error),
}

impl std::error::Error for AwfulError {
    fn description(&self) -> &str {
        match *self {
            AwfulError::Args(_) => "Usage error",
            AwfulError::NotImplemented => "Not implemented",
            AwfulError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            AwfulError::Io(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl fmt::Display for AwfulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AwfulError::Args(ref msg) => write!(f, "{}", msg),
            AwfulError::NotImplemented => write!(f, "Not implemented"),
            AwfulError::Io(ref err) => err.fmt(f),
        }
    }
}

pub struct FileBrowser {
    cur_path: PathBuf,
}

#[derive(Debug)]
pub struct File {
    name: ffi::OsString,
    file_type: fs::FileType,
}

impl File {
    pub fn new(e: Option<fs::DirEntry>) -> Option<File> {
        if e.is_none() {
            return None;
        }

        let e = e.unwrap();

        match e.file_type() {
            Err(_) => None,
            Ok(file_type) => {
                Some(File {
                         name: e.file_name(),
                         file_type: file_type,
                     })
            }
        }
    }

    pub fn name(&self) -> &ffi::OsStr {
        &self.name
    }

    pub fn file_type(&self) -> fs::FileType {
        self.file_type
    }
}

impl FileBrowser {
    pub fn new(path: &str) -> FileBrowser {
        FileBrowser { cur_path: PathBuf::from(path) }
    }
    pub fn cd(&mut self, rel_path: &str) {
        if rel_path == ".." {
            self.cur_path.pop();
        } else {
            self.cur_path.push(rel_path)
        }
    }

    pub fn list_files(&self) -> io::Result<Vec<File>> {
        let it = fs::read_dir(&self.cur_path)?;
        Ok(it.filter(|e| e.is_ok())
               .filter_map(|e| File::new(e.ok()))
               .collect())
    }

    pub fn path(&self) -> &PathBuf {
        &self.cur_path
    }

}
