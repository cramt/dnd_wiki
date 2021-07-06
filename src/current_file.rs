use once_cell::sync::Lazy;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;

pub static CURRENT_FILE: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::new()));

pub fn init<P: Into<PathBuf>>(p: P) {
    let mut curr = CURRENT_FILE.lock().unwrap();
    *curr = p.into()
}

pub fn open_new<S: AsRef<str>>(s: S) -> File {
    let curr = CURRENT_FILE.lock().unwrap();
    let path = curr.join(s.as_ref());
    File::open(path).unwrap()
}
