use std::path::PathBuf;
use std::env;
use std::io;
use std::ffi;
use std::fs;

struct FileBrowser {
    cur_path: PathBuf,
}

#[derive(Debug)]
struct File {
    pub name: ffi::OsString,
    pub file_type: fs::FileType,
}

impl File {
    fn new(e: Option<fs::DirEntry>) -> Option<File> {
        if e.is_none() {
          return None
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
    pub fn cd(&mut self, rel_path: &str) {
        if rel_path == ".." {
            self.cur_path.pop();
        } else {
            self.cur_path.push(rel_path)
        }
    }

    pub fn list_files(&self) -> io::Result<Vec<File>> {
        let it = fs::read_dir(&self.cur_path)?;
        Ok(it
          .filter(|e| e.is_ok())
          .filter_map(|e| File::new(e.ok()))
          .collect())
    }
}

fn main() {

    let mut args = env::args();
    args.next();

    let path = args.next();

    let fb = FileBrowser { cur_path: PathBuf::from(path.unwrap_or(String::from("."))) };

    let files = fb.list_files();

    // awful
    if let Ok(files) = files {
      for file in files.iter() {
        println!("{} {:?}",
        if file.file_type.is_dir() {"+"} else {"-"},
        file.name);
      }
    }

}
