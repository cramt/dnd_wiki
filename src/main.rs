use crate::current_file::{dist, init};
use crate::model::index::Index;
use crate::text_utils::path_normalize;
use clap::{AppSettings, Clap};
use once_cell::sync::Lazy;
use regex::Regex;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub mod current_file;
pub mod handlebars_definitions;
pub mod handlebars_engine;
pub mod model;
pub mod text_utils;

#[derive(Clap)]
#[clap(version = "1.0", author = "Alexandra Østermark <alex.cramt@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    path: Option<String>,
    #[clap(short, long)]
    out: Option<String>,
}

#[cfg(target_os = "windows")]
static ABSOLUTE_PATH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[A-Z]:\\.+").unwrap());
#[cfg(not(target_os = "windows"))]
static ABSOLUTE_PATH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("/.+").unwrap());

fn is_absolute_path<S: AsRef<str>>(s: S) -> bool {
    ABSOLUTE_PATH_REGEX.is_match(s.as_ref())
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut path = match opts.path {
        None => current_dir().unwrap(),
        Some(x) => match is_absolute_path(&x) {
            true => PathBuf::from(x),
            false => current_dir().unwrap().join(x),
        },
    }
    .canonicalize()
    .unwrap();
    if path.is_dir() {
        path = path.join("index.yaml")
    }
    if !path.exists() {
        println!("{:?} doesnt exists", path);
        return;
    }
    init(path.parent().unwrap());
    let index = serde_yaml::from_reader::<_, Index>(fs::File::open(path.clone()).unwrap()).unwrap();
    let out = dist(opts.out.unwrap_or("dist".to_string()));
    let _ = index.build()
        .unwrap()
        .into_iter()
        .map(|(path, body)| {
            let path = path_normalize(out.join(path.as_ref()));
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            File::create(path).unwrap().write(body.as_bytes()).unwrap()
        })
        .collect::<Vec<usize>>();
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy(
        path_normalize(path.parent().unwrap().join(index.static_folder)),
        out.clone(),
        &options,
    )
    .unwrap();
    let mut options = fs_extra::file::CopyOptions::new();
    options.overwrite = true;
    fs_extra::file::copy(
        path_normalize(path.parent().unwrap().join(index.style)),
        out.join("style.css"),
        &options,
    )
    .unwrap();
}
