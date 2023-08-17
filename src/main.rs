use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use walkdir::{DirEntry, WalkDir};

fn is_image_file(entry: &DirEntry) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?i)[.](?:jpe?g|png|webp|gif)$").unwrap();
    }
    entry
        .file_name()
        .to_str()
        .map(|s| RE.is_match(s))
        .unwrap_or(false)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|v| v.ok()) {
        if is_image_file(&entry) {
            println!("{}", entry.path().to_string_lossy());
        }
    }
}
