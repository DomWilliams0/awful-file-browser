use std::path::PathBuf;
use std::io;
use std::ffi;
use std::fs;


pub struct FileBrowser {
    cur_path: PathBuf,
}

#[derive(Debug)]
pub struct File {
    pub name: ffi::OsString,
    pub file_type: fs::FileType,
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
}
