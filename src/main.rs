use crate::current_file::init;
use crate::in_model::index::Index;
use crate::out_model::index::Index as OutIndex;
use crate::text_utils::path_normalize;
use clap::{AppSettings, Clap};
use notify::{RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use regex::Regex;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::Duration;

pub mod current_file;
pub mod handlebars_definitions;
pub mod handlebars_engine;
pub mod in_model;
pub mod into_model;
pub mod markdown;
pub mod out_model;
pub mod text_utils;

#[derive(Clap)]
#[clap(version = "1.0", author = "Alexandra Østermark <alex.cramt@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    path: Option<String>,
    #[clap(short, long)]
    out: Option<String>,
    #[clap(short, long)]
    watch: bool,
}

fn is_absolute_path<S: AsRef<str>>(s: S) -> bool {
    #[cfg(target_os = "windows")]
    static ABSOLUTE_PATH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[A-Z]:\\.+").unwrap());
    #[cfg(not(target_os = "windows"))]
    static ABSOLUTE_PATH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("/.+").unwrap());
    ABSOLUTE_PATH_REGEX.is_match(s.as_ref())
}

fn path_eval<S: AsRef<str>>(p: S) -> PathBuf {
    let p = p.as_ref();
    path_normalize(
        match is_absolute_path(p) {
            true => PathBuf::from(p),
            false => current_dir().unwrap().join(p),
        }
        .canonicalize()
        .unwrap(),
    )
}

fn main() {
    static OPTS: Lazy<Opts> = Lazy::new(|| Opts::parse());
    static IN_FILE: Lazy<PathBuf> = Lazy::new(|| {
        let in_file = path_eval(
            OPTS.deref()
                .path
                .clone()
                .unwrap_or_else(|| current_dir().unwrap().to_str().unwrap().to_string()),
        );
        let in_file = if in_file.is_dir() {
            in_file.join("index.yaml")
        } else {
            in_file
        };
        if !in_file.exists() {
            panic!("{:?} doesnt exists", in_file);
        }
        in_file
    });
    static ROOT_DIR: Lazy<PathBuf> = Lazy::new(|| IN_FILE.deref().parent().unwrap().to_path_buf());
    static OUT: Lazy<PathBuf> = Lazy::new(|| {
        OPTS.deref()
            .out
            .as_ref()
            .map(path_eval)
            .unwrap_or_else(|| ROOT_DIR.deref().join("dist").to_path_buf())
    });
    static WATCH: Lazy<bool> = Lazy::new(|| OPTS.deref().watch.clone());
    init(ROOT_DIR.as_path());
    if *WATCH.deref() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();

        watcher
            .watch(ROOT_DIR.deref(), RecursiveMode::Recursive)
            .unwrap();
        watcher.unwatch(OUT.deref()).unwrap();

        for res in rx {
            match res {
                notify::DebouncedEvent::Create(a)
                | notify::DebouncedEvent::Write(a)
                | notify::DebouncedEvent::Remove(a)
                | notify::DebouncedEvent::Rename(a, _) => {
                    if !a.starts_with(OUT.deref()) {
                        println!("building");
                        build(IN_FILE.clone(), ROOT_DIR.clone(), OUT.clone())
                    }
                }
                notify::DebouncedEvent::Error(a, b) => {
                    println!("{:?} caused error {:?}", b, a)
                }
                _ => {}
            }
        }
    } else {
        build(IN_FILE.clone(), ROOT_DIR.clone(), OUT.clone())
    }
}

fn build(in_file: PathBuf, root_dir: PathBuf, out: PathBuf) {
    let index =
        serde_yaml::from_reader::<_, Index>(fs::File::open(in_file.clone()).unwrap()).unwrap();
    let index: OutIndex = index.into();
    let finish = index.build().unwrap();
    fs_extra::dir::remove(out.as_path()).unwrap();
    let _ = finish
        .into_iter()
        .map(|(path, body)| {
            let path = out.join(path.as_ref());
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            File::create(path).unwrap().write(body.as_bytes()).unwrap()
        })
        .collect::<Vec<usize>>();
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy(root_dir.join(index.static_folder), out.clone(), &options).unwrap();
    let mut options = fs_extra::file::CopyOptions::new();
    options.overwrite = true;
    fs_extra::file::copy(root_dir.join(index.style), out.join("style.css"), &options).unwrap();
}
