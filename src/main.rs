extern crate awful_files;

use awful_files::{FileBrowser, File};

fn main() {
    let fb = FileBrowser::new(".");

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
