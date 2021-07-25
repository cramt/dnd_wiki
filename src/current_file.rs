use once_cell::sync::Lazy;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;

pub static CURRENT_FILE: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::new()));

pub fn init<P: Into<PathBuf>>(p: P) {
    let mut curr = CURRENT_FILE.lock().unwrap();
    *curr = p.into()
}

pub fn join<S: AsRef<str>>(s: S) -> PathBuf {
    let curr = CURRENT_FILE.lock().unwrap();
    curr.join(s.as_ref())
}

pub fn open_new<S: AsRef<str>>(s: S) -> File {
    let path = join(s);
    File::open(path).unwrap()
}
